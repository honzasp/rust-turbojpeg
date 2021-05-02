use std::convert::TryInto as _;
use std::{ptr, slice};
use crate::{Image, sys};
use crate::common::{Subsamp, Result, Error, get_error};

/// Compresses raw pixel data into JPEG.
#[derive(Debug)]
pub struct Compressor {
    handle: sys::tjhandle,
    quality: i32,
    subsamp: Subsamp,
}

static DEFAULT_QUALITY: i32 = 95;
static DEFAULT_SUBSAMP: Subsamp = Subsamp::None;

unsafe impl Send for Compressor {}

#[derive(Debug)]
struct CompressBuf {
    ptr: *mut u8,
    len: usize,
    owned: bool,
}

impl Compressor {
    /// Create a new compressor instance.
    #[doc(alias = "tjInitCompress")]
    pub fn new() -> Result<Compressor> {
        unsafe {
            let handle = sys::tjInitCompress();
            if !handle.is_null() {
                Ok(Compressor {
                    handle,
                    quality: DEFAULT_QUALITY,
                    subsamp: DEFAULT_SUBSAMP,
                })
            } else {
                Err(get_error(handle))
            }
        }
    }

    /// Set the quality of the compressed JPEG images.
    ///
    /// The quality ranges from 1 (worst) to 100 (best).
    pub fn set_quality(&mut self, quality: i32) {
        // TODO: check whether 1 <= quality <= 100 here?
        self.quality = quality;
    }

    /// Set the level of chrominance subsampling of the compressed JPEG images.
    ///
    /// Chrominance subsampling can reduce the compressed image size without noticeable loss of
    /// quality (see [`Subsamp`] for more).
    pub fn set_subsamp(&mut self, subsamp: Subsamp) {
        self.subsamp = subsamp;
    }

    fn raw_compress(&mut self, image: Image<&[u8]>, buf: &mut CompressBuf) -> Result<()> {
        image.assert_valid(image.pixels.len());

        let Image { pixels, width, pitch, height, format } = image;
        let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let pitch = pitch.try_into().map_err(|_| Error::IntegerOverflow("pitch"))?;
        let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;

        let mut buf_len = buf.len as libc::c_ulong;
        let res = unsafe {
            sys::tjCompress2(
                self.handle,
                pixels.as_ptr(), width, pitch, height, format as i32,
                &mut buf.ptr, &mut buf_len,
                self.subsamp as i32, self.quality,
                if buf.owned { 0 } else { sys::TJFLAG_NOREALLOC } as i32
            )
        };
        buf.len = buf_len as usize;

        if res != 0 {
            Err(unsafe { get_error(self.handle) })
        } else if buf.ptr.is_null() {
            Err(Error::Null())
        } else {
            Ok(())
        }
    }

    /// Compress the `image` into a new `Vec<u8>`.
    ///
    /// This method is simpler than [`compress_to_slice`](Compressor::compress_to_slice), but it
    /// requires multiple allocations and copies the output data from internal buffer to a `Vec`.
    #[doc(alias = "tjCompress2")]
    #[doc(alias = "tjCompress")]
    pub fn compress_to_vec(&mut self, image: Image<&[u8]>) -> Result<Vec<u8>> {
        let mut buf = CompressBuf { ptr: ptr::null_mut(), len: 0, owned: true };
        self.raw_compress(image, &mut buf)?;
        let buf_slice = unsafe { slice::from_raw_parts(buf.ptr, buf.len) };
        Ok(buf_slice.to_vec())
    }

    /// Compress the `image` into the buffer `dest`.
    ///
    /// Returns the size of the compressed image. If the compressed image does not fit into `dest`,
    /// this method returns an error. Use [`buf_len`](Compressor::buf_len) to determine buffer size
    /// that is guaranteed to be large enough to compress the `image`.
    #[doc(alias = "tjCompress2")]
    #[doc(alias = "tjCompress")]
    pub fn compress_to_slice(&mut self, image: Image<&[u8]>, dest: &mut [u8]) -> Result<usize> {
        let mut buf = CompressBuf { ptr: dest.as_mut_ptr(), len: dest.len(), owned: false };
        self.raw_compress(image, &mut buf)?;
        Ok(buf.len)
    }

    /// Compute the maximum size of a compressed image.
    ///
    /// This depends on image `width` and `height`, and also on the current setting of chrominance
    /// subsampling (see [`set_subsamp()`](Compressor::set_subsamp).
    pub fn buf_len(&self, width: usize, height: usize) -> Result<usize> {
        let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
        let len = unsafe { sys::tjBufSize(width, height, self.subsamp as i32) };
        let len = len.try_into().map_err(|_| Error::IntegerOverflow("buf len"))?;
        Ok(len)
    }
}

impl Drop for Compressor {
    fn drop(&mut self) {
        unsafe { sys::tjDestroy(self.handle); }
    }
}

impl Drop for CompressBuf {
    fn drop(&mut self) {
        if self.owned {
            unsafe { sys::tjFree(self.ptr) };
        }
    }
}
