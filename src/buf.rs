use std::{ptr, slice};
use std::convert::{AsRef, AsMut};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Owned buffer with JPEG data.
///
/// This represents a memory slice which is owned by TurboJPEG and can be automatically resized.
#[derive(Debug)]
pub struct OwnedBuf {
    ptr: *mut u8,
    len: usize,
}

impl Deref for OwnedBuf {
    type Target = [u8];
    fn deref(&self) -> &[u8] { unsafe { deref(self.ptr, self.len) } }
}
impl DerefMut for OwnedBuf {
    fn deref_mut(&mut self) -> &mut [u8] { unsafe { deref_mut(self.ptr, self.len) } }
}
impl AsRef<[u8]> for OwnedBuf {
    fn as_ref(&self) -> &[u8] { self.deref() }
}
impl AsMut<[u8]> for OwnedBuf {
    fn as_mut(&mut self) -> &mut [u8] { self.deref_mut() }
}

impl OwnedBuf {
    /// Creates an empty buffer.
    pub fn new() -> OwnedBuf {
        OwnedBuf { ptr: ptr::null_mut(), len: 0 }
    }

    /// Allocates a buffer with given length.
    ///
    /// Panics if `len` overflows or if the memory cannot be allocated.
    pub fn allocate(len: usize) -> OwnedBuf {
        let ptr = unsafe { raw::tjAlloc(len as libc::c_int) };
        assert!(!ptr.is_null(), "tjAlloc() returned null");
        OwnedBuf { ptr, len }
    }

    /// Creates a new buffer copied from a slice.
    pub fn copy_from_slice(data: &[u8]) -> OwnedBuf {
        let buf = Self::allocate(data.len());
        unsafe { ptr::copy_nonoverlapping(data.as_ptr(), buf.ptr, data.len()) }
        buf
    }

    /// Returns the length of the buffer.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for OwnedBuf {
    fn drop(&mut self) {
        unsafe { raw::tjFree(self.ptr) };
    }
}



/// Output buffer for JPEG data.
///
/// When compressing or transforming images, we need a memory buffer to store the compressed JPEG
/// data. This buffer comes in two variants, which are similar to `Cow::Borrowed` and `Cow::Owned`
/// from the standard library:
///
/// - Borrowed buffer wraps a `&mut [u8]`, preallocated slice of fixed size provided by you. When
/// using a borrowed buffer, TurboJPEG cannot resize the buffer, so the operation will fail with an
/// error if the output does not fit into the buffer.
///
/// - Owned buffer wraps an [`OwnedBuf`], memory buffer owned by TurboJPEG. This buffer can be
/// automatically resized to contain the compressed data, so you don't have to worry about its size.
///
/// The lifetime parameter `'a` tracks the lifetime of the borrowed slice. In the case of owned
/// buffer, the lifetime can be `'static`.
#[derive(Debug)]
pub struct OutputBuf<'a> {
    pub(crate) ptr: *mut u8,
    pub(crate) len: usize,
    pub(crate) is_owned: bool,
    pub(crate) _phantom: PhantomData<&'a mut [u8]>,
}

impl<'a> Deref for OutputBuf<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] { unsafe { deref(self.ptr, self.len) } }
}
impl<'a> DerefMut for OutputBuf<'a> {
    fn deref_mut(&mut self) -> &mut [u8] { unsafe { deref_mut(self.ptr, self.len) } }
}
impl<'a> AsRef<[u8]> for OutputBuf<'a> {
    fn as_ref(&self) -> &[u8] { self.deref() }
}
impl<'a> AsMut<[u8]> for OutputBuf<'a> {
    fn as_mut(&mut self) -> &mut [u8] { self.deref_mut() }
}


impl<'a> OutputBuf<'a> {
    /// Converts a slice into a borrowed `OutputBuf`.
    pub fn borrowed(slice: &'a mut [u8]) -> OutputBuf<'a> {
        OutputBuf {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
            is_owned: false,
            _phantom: PhantomData,
        }
    }

    /// Converts an `OwnedBuf` into an owned `OutputBuf`.
    pub fn owned(mut buf: OwnedBuf) -> OutputBuf<'a> {
        let OwnedBuf { ptr, len } = buf;
        buf.ptr = ptr::null_mut(); // do not free the pointer in the OwnedBuf destructor
        OutputBuf {
            ptr,
            len,
            is_owned: true,
            _phantom: PhantomData,
        }
    }

    /// Creates an empty owned buffer.
    pub fn new_owned() -> OutputBuf<'a> {
        Self::owned(OwnedBuf::new())
    }

    /// Allocates an owned buffer with given capacity.
    pub fn allocate_owned(cap: usize) -> OutputBuf<'a> {
        Self::owned(OwnedBuf::allocate(cap))
    }

    /// Returns the length of the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Converts this buffer into an owned buffer.
    ///
    /// If `self` is owned, this is a trivial operation, otherwise we must copy the data from the
    /// borrowed slice into a new owned buffer.
    pub fn into_owned(mut self) -> OwnedBuf {
        let OutputBuf { ptr, len, is_owned, .. } = self;
        self.ptr = ptr::null_mut(); // do not free the pointer in OutputBuf destructor
        if is_owned {
            OwnedBuf { ptr, len }
        } else {
            unsafe {
                OwnedBuf::copy_from_slice(slice::from_raw_parts(ptr, len))
            }
        }
    }
}

impl<'a> Drop for OutputBuf<'a> {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe { raw::tjFree(self.ptr) };
        }
    }
}

impl<'a> From<&'a mut [u8]> for OutputBuf<'a> {
    fn from(slice: &'a mut [u8]) -> OutputBuf<'a> {
        OutputBuf::borrowed(slice)
    }
}

impl From<OwnedBuf> for OutputBuf<'static> {
    fn from(buf: OwnedBuf) -> OutputBuf<'static> {
        OutputBuf::owned(buf)
    }
}

unsafe fn deref<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if len != 0 {
        debug_assert!(!ptr.is_null());
        slice::from_raw_parts(ptr, len)
    } else {
        &[]
    }
}

unsafe fn deref_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
    if len != 0 {
        debug_assert!(!ptr.is_null());
        slice::from_raw_parts_mut(ptr, len)
    } else {
        &mut []
    }
}
