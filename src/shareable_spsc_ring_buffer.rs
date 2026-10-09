use std::cell::UnsafeCell;
use crate::ck;
use crate::error_types::DetailedError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr;
use std::rc::Rc;

#[derive(Debug)]
pub enum ShareableSpscRingBufferError {
    SizeIsZero,
    IncompatibleSize {
        size: usize,
        usize_size: usize,
    },
    CacheLinePairSizeIsZero,
    MisalignedCacheLinePairsRing {
        cache_line_pair_size: usize,
        ring_alignment: usize,
    },
    MisalignedCacheLinePairsUsize {
        cache_line_pair_size: usize,
        usize_alignment: usize,
    },
    DestinationMisaligned {
        cache_line_pair_size: usize,
        dest_addr: usize,
    },
    RingBufferFull,
}

impl Display for ShareableSpscRingBufferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SizeIsZero => {
                write!(f, "SIZE must not be zero")
            }
            Self::IncompatibleSize { size, usize_size } => {
                write!(
                    f,
                    "SIZE ({size}) must be a multiple of size_of::<usize>() ({usize_size}) and SIZE / size_of::<usize>() must be a power of 2 in [4, u32::MAX]."
                )
            }
            Self::CacheLinePairSizeIsZero => {
                write!(f, "CACHE_LINE_PAIR_SIZE must not be zero.")
            }
            Self::MisalignedCacheLinePairsRing {
                cache_line_pair_size,
                ring_alignment,
            } => {
                write!(
                    f,
                    "CACHE_LINE_PAIR_SIZE ({cache_line_pair_size}) must be a multiple of align_of::<UnsafeCell<ck::ck_ring_t>>() ({ring_alignment})."
                )
            }
            Self::MisalignedCacheLinePairsUsize {
                cache_line_pair_size,
                usize_alignment,
            } => {
                write!(
                    f,
                    "CACHE_LINE_PAIR_SIZE ({cache_line_pair_size}) must be a multiple of align_of::<usize>() ({usize_alignment})."
                )
            }
            Self::DestinationMisaligned {
                cache_line_pair_size,
                dest_addr,
            } => {
                write!(
                    f,
                    "Destination ({dest_addr:x}) must be aligned to CACHE_LINE_PAIR_SIZE ({cache_line_pair_size})."
                )
            }
            Self::RingBufferFull => {
                write!(f, "The ring buffer is full.")
            }
        }
    }
}

impl Error for ShareableSpscRingBufferError {}

/// # Notes
/// - `SIZE` must be non-zero
/// - `SIZE` must be a multiple of `size_of::<usize>()`
/// - `SIZE / size_of::<usize>()` must not be greater than `u32::MAX`
/// - `SIZE / size_of::<usize>()` must be at least 4
/// - `SIZE / size_of::<usize>()` must be a power of 2
/// - `CACHE_LINE_PAIR_SIZE` must be non-zero
/// - `CACHE_LINE_PAIR_SIZE` must be a multiple of `align_of::<UnsafeCell<ck::ck_ring_t>>()`
/// - `CACHE_LINE_PAIR_SIZE` must be a multiple of `align_of::<usize>()`
#[repr(C)]
pub struct ShareableSpscRingBuffer<const SIZE: usize, const CACHE_LINE_PAIR_SIZE: usize> {
    ring: UnsafeCell<ck::ck_ring_t>,
    _mem0: UnsafeCell<[u8; CACHE_LINE_PAIR_SIZE]>,
    _mem1: UnsafeCell<[u8; SIZE]>,
    _marker: PhantomData<Rc<()>>
}

impl<const SIZE: usize, const CACHE_LINE_PAIR_SIZE: usize>
    ShareableSpscRingBuffer<SIZE, CACHE_LINE_PAIR_SIZE>
{
    /// # Notes
    /// - `dest` must be aligned to `CACHE_LINE_PAIR_SIZE`
    /// - on `Err`, the caller must treat dest as uninitialized.
    pub fn init(dest: Pin<&mut MaybeUninit<Self>>) -> Result<&Self, DetailedError> {
        if SIZE == 0 {
            return Err(DetailedError::add_metadata(
                ShareableSpscRingBufferError::SizeIsZero,
            ));
        }

        if !SIZE.is_multiple_of(size_of::<usize>())
            || SIZE / size_of::<usize>() > u32::MAX as usize
            || SIZE / size_of::<usize>() < 4
            || !(SIZE / size_of::<usize>()).is_power_of_two() {
            return Err(DetailedError::add_metadata(
                ShareableSpscRingBufferError::IncompatibleSize {
                    size: SIZE,
                    usize_size: size_of::<usize>(),
                },
            ));
        }

        if CACHE_LINE_PAIR_SIZE == 0 {
            return Err(DetailedError::add_metadata(
                ShareableSpscRingBufferError::CacheLinePairSizeIsZero,
            ));
        }

        if !CACHE_LINE_PAIR_SIZE.is_multiple_of(align_of::<ck::ck_ring_t>()) {
            return Err(DetailedError::add_metadata(
                ShareableSpscRingBufferError::MisalignedCacheLinePairsRing {
                    cache_line_pair_size: CACHE_LINE_PAIR_SIZE,
                    ring_alignment: align_of::<ck::ck_ring_t>(),
                },
            ));
        }

        if !CACHE_LINE_PAIR_SIZE.is_multiple_of(align_of::<usize>()) {
            return Err(DetailedError::add_metadata(
                ShareableSpscRingBufferError::MisalignedCacheLinePairsUsize {
                    cache_line_pair_size: CACHE_LINE_PAIR_SIZE,
                    usize_alignment: align_of::<usize>(),
                },
            ));
        }

        if !dest.as_ptr().addr().is_multiple_of(CACHE_LINE_PAIR_SIZE) {
            return Err(DetailedError::add_metadata(
                ShareableSpscRingBufferError::DestinationMisaligned {
                    cache_line_pair_size: CACHE_LINE_PAIR_SIZE,
                    dest_addr: dest.as_ptr().addr(),
                },
            ));
        }

        // SAFETY: The pinned value won't move as it isn't moved inside this function and the
        // pointer is discarded upon returning
        let dest = unsafe { dest.get_unchecked_mut() }
            .as_mut_ptr()
            .cast::<Self>();

        // SAFETY: It is safe to write to the ring. Pointer is large enough and aligned.
        //    dest aligns with the ring field in the struct.
        unsafe {
            #[allow(clippy::cast_possible_truncation)] // We checked this
            ck::ck_ring_init(dest.cast(), (SIZE / size_of::<usize>()) as u32);
        }

        // SAFETY: Initialization can't fail
        Ok(unsafe { dest.as_ref_unchecked() })
    }

    fn ptr_to_buffer(&self) -> &UnsafeCell<[u8; SIZE]> {
        let mut buffer = (&raw const self._mem0).cast::<u8>();
        if !buffer.addr().is_multiple_of(CACHE_LINE_PAIR_SIZE) {
            // SAFETY: Address can't wrap because the upper half of
            //   the address space is reserved to the Linux kernel. The constructors guarantee that the
            //   buffer is large enough
            unsafe {
                buffer = buffer.add(CACHE_LINE_PAIR_SIZE - (buffer.addr() % CACHE_LINE_PAIR_SIZE));
            }
        }

        // SAFETY: all bit patterns are valid, no mutable references exist (no interface for that)
        unsafe { buffer.cast::<UnsafeCell<[u8; SIZE]>>().as_ref_unchecked() }
    }

    /// # Safety
    /// todo
    #[inline(always)]
    pub unsafe fn enqueue(&self, value: usize) -> Result<(), ShareableSpscRingBufferError> {
        match unsafe {
            ck::ck_ring_enqueue_spsc(self.ring.get(), self.ptr_to_buffer().get().cast(), ptr::without_provenance(value))
        } {
            true => Ok(()),
            false => Err(ShareableSpscRingBufferError::RingBufferFull),
        }
    }

    /// # Safety
    /// todo
    #[inline(always)]
    pub unsafe fn dequeue(&self) -> Option<usize> {
        let mut value = MaybeUninit::<usize>::uninit();
        match unsafe {
            ck::ck_ring_dequeue_spsc(self.ring.get(), self.ptr_to_buffer().get().cast(), value.as_mut_ptr().cast())
        } {
            true => Some(unsafe { value.assume_init() }),
            false => None,
        }
    }
}