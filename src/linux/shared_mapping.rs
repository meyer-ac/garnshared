use nix::errno::Errno;
use nix::fcntl::{FcntlArg, SealFlag, fcntl};
use nix::sys::mman::{MapFlags, ProtFlags, mmap, munmap};
use nix::sys::stat::fstat;
use std::error::Error;
use std::ffi::c_void;
use std::fmt::Display;
use std::num::NonZero;
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};
use std::ptr::NonNull;

#[derive(Debug)]
pub enum SharedMappingError {
    WrongFileSize,
    FdNotSealed,
    GetFileSealsFailed(Errno),
    GetFileStatsFailed(Errno),
    MapMemoryFailed(Errno),
}

impl Display for SharedMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::WrongFileSize => {
                write!(
                    f,
                    "The provided file is not the same size as the requested block of memory to be mapped."
                )
            }
            Self::FdNotSealed => {
                write!(
                    f,
                    "The provided file descriptor did not have the necessary seals."
                )
            }
            Self::GetFileSealsFailed(e) => {
                write!(f, "Failed to get file seals with errno {e}")
            }
            Self::GetFileStatsFailed(e) => {
                write!(f, "Failed to get file stats with errno {e}")
            }
            Self::MapMemoryFailed(e) => {
                write!(f, "Failed to map the file into memory with errno {e}")
            }
        }
    }
}

impl Error for SharedMappingError {}

/// A minimal wrapper around `mmap` that unmaps the memory on drop
pub struct SharedMapping {
    ptr: NonNull<c_void>,
    size: NonZero<usize>,
    fd: OwnedFd,
}

impl SharedMapping {
    pub fn new(fd: OwnedFd, size: NonZero<usize>) -> Result<Self, SharedMappingError> {
        let seals = match fcntl(fd.as_fd(), FcntlArg::F_GET_SEALS) {
            Ok(val) => val,
            Err(Errno::EINVAL) => return Err(SharedMappingError::FdNotSealed),
            Err(e) => return Err(SharedMappingError::GetFileSealsFailed(e)),
        };
        if !SealFlag::from_bits_truncate(seals)
            .contains(SealFlag::F_SEAL_SEAL | SealFlag::F_SEAL_GROW | SealFlag::F_SEAL_SHRINK)
        {
            return Err(SharedMappingError::FdNotSealed);
        }

        let stats = fstat(fd.as_fd()).map_err(SharedMappingError::GetFileStatsFailed)?;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        // Not gonna happen on a 64-bit system
        if size.get() != stats.st_size as usize {
            return Err(SharedMappingError::WrongFileSize);
        }

        // SAFETY: The resource is `size` bytes large,
        // `prot` and `flags` are only passed valid flags,
        // `offset` is trivially a multiple of the system's page size and
        // `addr` is omitted.
        let ptr = unsafe {
            mmap(
                None,
                size,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED,
                fd.as_fd(),
                0,
            )
        }
        .map_err(SharedMappingError::MapMemoryFailed)?;

        Ok(SharedMapping { ptr, size, fd })
    }

    #[must_use]
    pub fn as_ptr(&self) -> NonNull<c_void> {
        self.ptr
    }
}

impl AsFd for SharedMapping {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl Drop for SharedMapping {
    fn drop(&mut self) {
        // SAFETY: The resource is `size` bytes large,
        // `addr` being a multiple of the page size is guaranteed by `mmap`, which aligns
        // the memory to page boundaries
        let result = unsafe { munmap(self.ptr, self.size.get()) };
        debug_assert_eq!(result, Ok(()), "munmap() failed");
    }
}
