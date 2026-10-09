use std::error::Error;
use std::ffi::c_void;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::num::NonZero;
use std::ops::Deref;
use std::os::fd::{AsFd, OwnedFd};
use std::pin::Pin;
use std::{mem, ptr};
use std::ptr::NonNull;
use hashed_type_def::add_hash_fnv1a;
use nix::fcntl::{fcntl, FcntlArg, SealFlag};
use nix::libc::off_t;
use nix::sys::memfd::{memfd_create, MFdFlags};
use nix::unistd::{ftruncate, sysconf, SysconfVar};
use uuid::Uuid;
use crate::constants;
use crate::error_types::{DetailedError, ResultMetadata};
use crate::linux::traits::ShmCompatible;
use crate::linux::shared_mapping::{SharedMapping, SharedMappingError};

#[derive(Debug)]
pub enum ShmBoxError {
    TypeNotNonDrop,
    GetPageSizeFailed,
    IncompatibleAlignment,
    SuccessfulInitializationNotProven,
    WrongType,
    MappingMemoryFailed(SharedMappingError),
}

impl Display for ShmBoxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeNotNonDrop => f.write_str("The pointee type must be trivially destructible."),
            Self::GetPageSizeFailed => {
                write!(f, "Failed to get the system's page size.")
            }
            Self::IncompatibleAlignment => {
                write!(f, "The system's page size is not a multiple of the alignment of Uuid and the least common multiple of the alignment of the pointee type and the size of a cache line pair.")
            }
            Self::SuccessfulInitializationNotProven => {
                write!(f, "The successful initialization of the pointee was not proven, i.e. a reference to the pointee was not returned from the placement constructor.")
            }
            Self::WrongType => {
                write!(f, "The opened ShmBox holds another type than that requested.")
            }
            Self::MappingMemoryFailed(e) => {
                write!(f, "Failed to map the file into memory: {e}")
            }
        }
    }
}

impl Error for ShmBoxError {}

/// # INVARIANT
/// The system's page size is a multiple of the alignment of `Uuid` and the least common multiple of the alignment of the pointee type and the size of a cache line pair.
pub struct ShmBox<T: ShmCompatible, const CACHE_LINE_PAIR_SIZE: usize> {
    mapping: SharedMapping,
    _marker: PhantomData<T>,
}

impl<T: ShmCompatible, const CACHE_LINE_PAIR_SIZE: usize> ShmBox<T, CACHE_LINE_PAIR_SIZE> {
    const LAYOUT_VERSION: u32 = 1;

    /// This uuid identifies both the pointee type (with size and alignment) and the cache line pair size
    const fn type_uuid() -> Uuid {
        let mut uuid = T::TYPE_HASH_LE;
        uuid = add_hash_fnv1a(&CACHE_LINE_PAIR_SIZE.to_le_bytes(), uuid);
        uuid = add_hash_fnv1a(&size_of::<T>().to_le_bytes(), uuid);
        uuid = add_hash_fnv1a(&align_of::<T>().to_le_bytes(), uuid);
        uuid = add_hash_fnv1a(&size_of::<Uuid>().to_le_bytes(), uuid);
        uuid = add_hash_fnv1a(&align_of::<Uuid>().to_le_bytes(), uuid);
        uuid = add_hash_fnv1a(&Self::LAYOUT_VERSION.to_le_bytes(), uuid);
        Uuid::from_u128(uuid)
    }

    const fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    const fn lcm(a: NonZero<usize>, b: NonZero<usize>) -> NonZero<usize> {
        // Unwrap: LCM can't be zero because `a` and `b` are non-zero
        NonZero::new((a.get() / Self::gcd(a.get(), b.get())) * b.get()).unwrap()
    }

    const T_ALIGNMENT: NonZero<usize> = {
        if CACHE_LINE_PAIR_SIZE == 0 {
            // alignment is at least 1
            NonZero::new(align_of::<T>()).unwrap()
        } else {
            // alignment is at least 1, `CACHE_LINE_PAIR_SIZE` is not zero (see above)
            Self::lcm(NonZero::new(align_of::<T>()).unwrap(), NonZero::new(CACHE_LINE_PAIR_SIZE).unwrap())
        }
    };

    const DATA_OFFSET: usize = {
        let t_alignment = Self::T_ALIGNMENT.get();
        let mut offset = size_of::<Uuid>();
        if !offset.is_multiple_of(t_alignment) {
            offset += t_alignment - offset % t_alignment;
        }
        offset
    };

    const BUFFER_SIZE: NonZero<usize> = {
        // MEMORY LAYOUT:
        // - alignment padding
        // - `Uuid`
        // - alignment + cache line pair padding
        // - `T`
        // - next cache line pair padding (optional)

        let mut size = 0usize;
        size += Self::DATA_OFFSET;
        size += size_of::<T>();
        if CACHE_LINE_PAIR_SIZE != 0 && !size.is_multiple_of(CACHE_LINE_PAIR_SIZE) {
            size += CACHE_LINE_PAIR_SIZE - (size % CACHE_LINE_PAIR_SIZE);
        }
        NonZero::new(size).unwrap() // compile-time unwrap, fine
    };

    /// Note: The returned pointer is properly aligned
    fn ptr_to_type_uuid(mem: NonNull<c_void>) -> NonNull<Uuid> {
        // The alignment is guaranteed by the invariant
        mem.cast::<_>()
    }

    /// Note: The returned pointer is properly aligned
    fn ptr_to_data(mem: NonNull<c_void>) -> NonNull<T> {
        let mem = mem.cast::<u8>();

        // SAFETY: Address was non-null in the beginning and can't wrap because the upper half of
        //   the address space is reserved to the Linux kernel.
        unsafe { mem.add(Self::DATA_OFFSET) }.cast()
    }

    fn check_page_size() -> Result<(), ShmBoxError> {
        let page_size = match sysconf(SysconfVar::PAGE_SIZE) {
            Ok(Some(0) | None) | Err(_) => return Err(ShmBoxError::GetPageSizeFailed),
            Ok(Some(res)) => usize::try_from(res).map_err(|_| ShmBoxError::GetPageSizeFailed)?,
        };

        if !page_size.is_multiple_of(Self::T_ALIGNMENT.get()) || !page_size.is_multiple_of(align_of::<Uuid>()) {
            return Err(ShmBoxError::IncompatibleAlignment)
        }

        Ok(())
    }

    /// `placement_constructor` must return a reference to the object it just constructed
    pub fn new<F>(placement_constructor: F) -> Result<Self, DetailedError>
    where
        F: FnOnce(Pin<&mut MaybeUninit<T>>) -> Result<&T, DetailedError>,
    {
        if mem::needs_drop::<T>() {
            return Err(DetailedError::add_metadata(ShmBoxError::TypeNotNonDrop));
        }

        Self::check_page_size().add_metadata()?;

        let shm_fd = memfd_create(
            constants::SHM_FILE_NAME,
            MFdFlags::MFD_CLOEXEC | MFdFlags::MFD_ALLOW_SEALING
        ).add_metadata()?;

        ftruncate(shm_fd.as_fd(), off_t::try_from(Self::BUFFER_SIZE.get()).add_metadata()?).add_metadata()?;

        fcntl(
            shm_fd.as_fd(),
            FcntlArg::F_ADD_SEALS(
                SealFlag::F_SEAL_SEAL | SealFlag::F_SEAL_SHRINK | SealFlag::F_SEAL_GROW,
            ),
        ).add_metadata()?;

        let mapping = SharedMapping::new(shm_fd, Self::BUFFER_SIZE).add_metadata()?;

        // SAFETY: The size of the pointer is guaranteed by the `ftruncate` call with `BUFFER_SIZE`,
        // the alignment is guaranteed by the call to `ptr_to_type_uuid`. At this point in time, no
        // immutable references to the pointee exist.
        unsafe {
            Self::ptr_to_type_uuid(mapping.as_ptr()).as_ptr().write(Self::type_uuid());
        }

        // SAFETY: The size of the pointer is guaranteed by the `ftruncate` call with `BUFFER_SIZE`,
        // the alignment is guaranteed by the call to `ptr_to_data`
        let dest = unsafe {
            Self::ptr_to_data(mapping.as_ptr()).cast::<MaybeUninit<T>>().as_mut()
        };
        let dest_ptr = dest.as_ptr();

        // SAFETY: ShmBox does not provide a way to move the pinned value through its safe interface
        let returned_ref = placement_constructor(unsafe { Pin::new_unchecked(dest) })?;

        // [*] If the placement constructor returned a reference to the same memory location that
        // `dest`points to, we have obtained a reference to `T` at that memory location from safe
        // Rust. It is therefore safe to assume that the pointee of dest has been initialized after
        // this check.
        if dest_ptr != ptr::from_ref(returned_ref) {
            return Err(DetailedError::add_metadata(ShmBoxError::SuccessfulInitializationNotProven));
        }

        Ok(Self {
            mapping,
            _marker: PhantomData,
        })
    }

    /// # Safety
    /// `fd` references a block of shared memory that was initialized by the constructor of `ShmBox`
    pub unsafe fn from_fd(fd: OwnedFd) -> Result<Self, ShmBoxError> {
        if mem::needs_drop::<T>() {
            return Err(ShmBoxError::TypeNotNonDrop);
        }

        Self::check_page_size()?;

        let mapping = SharedMapping::new(fd, Self::BUFFER_SIZE).map_err(ShmBoxError::MappingMemoryFailed)?;

        // SAFETY: The safety comment guarantees that `mem` originates from the constructor of `ShmBox` and
        // therefore has the right memory layout. Furthermore, the interface of `ShmBox` exposes no
        // references (in particular no mutable ones) to the type uuid.
        let type_uuid = unsafe {
            Self::ptr_to_type_uuid(mapping.as_ptr()).read()
        };

        if type_uuid != Self::type_uuid() {
            return Err(ShmBoxError::WrongType);
        }

        Ok(Self {
            mapping,
            _marker: PhantomData,
        })
    }

    /// # Safety
    /// The memory that the returned fd points to may not be used for anything else but passing it
    /// to `ShmBox::from_fd` or converting it to a `RawFd` and sending it over a socket as auxiliary
    /// data.
    pub unsafe fn generate_fd(&self) -> Result<OwnedFd, DetailedError> {
        self.mapping.as_fd().try_clone_to_owned().add_metadata()
    }
}

impl<T: ShmCompatible, const CACHE_LINE_PAIR_SIZE: usize> Deref for ShmBox<T, CACHE_LINE_PAIR_SIZE> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The safety comment guarantees that `mem` originates from the constructor of `ShmBox` and
        // therefore has the right memory layout and the data has been initialized (see [*]). Furthermore,
        // the interface of `ShmBox` guarantees that no mutable references exist at the same time.
        // Lastly, the type has been checked either at compile time or in `ShmBox::from_fd`.
        unsafe {
            Self::ptr_to_data(self.mapping.as_ptr()).cast::<T>().as_ref()
        }
    }
}

// SAFETY: T is Sync and we only expose &T. We also own the mapping.
unsafe impl<T: ShmCompatible, const N: usize> Send for ShmBox<T, N> {}
unsafe impl<T: ShmCompatible, const N: usize> Sync for ShmBox<T, N> {}