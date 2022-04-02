use std::convert::TryInto as _;
use crate::{Image, sys};
use crate::buf::{OwnedBuf, OutputBuf};
use crate::common::{Subsamp, Result, Error, get_error};

/// Compresses raw pixel data into JPEG.
#[derive(Debug)]
#[doc(alias = "tjhandle")]
pub struct Compressor {
    handle: sys::tjhandle,
    quality: i32,
    subsamp: Subsamp,
}

static DEFAULT_QUALITY: i32 = 95;
static DEFAULT_SUBSAMP: Subsamp = Subsamp::None;

unsafe impl Send for Compressor {}

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

    /// Compresses the `image` into a buffer.
    ///
    /// This is the main compression method, which gives you full control of the output buffer. If
    /// you don't need this level of control, you can use [`compress_to_vec`].
    #[doc(alias = "tjCompress2")]
    #[doc(alias = "tjCompress")]
    pub fn compress(&mut self, image: Image<&[u8]>, buf: &mut OutputBuf) -> Result<()> {
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
                if buf.is_owned { 0 } else { sys::TJFLAG_NOREALLOC } as i32
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

    /// Compresses the `image` into an owned buffer.
    ///
    /// This method automatically allocates the memory and avoids needless copying.
    pub fn compress_to_owned(&mut self, image: Image<&[u8]>) -> Result<OwnedBuf> {
        let mut buf = OutputBuf::new_owned();
        self.compress(image, &mut buf)?;
        Ok(buf.into_owned())
    }

    /// Compress the `image` into a new `Vec<u8>`.
    ///
    /// This method copies the compressed data into a new `Vec`. If you would like to avoid the
    /// extra allocation and copying, consider using [`compress_to_owned`] instead.
    pub fn compress_to_vec(&mut self, image: Image<&[u8]>) -> Result<Vec<u8>> {
        let mut buf = OutputBuf::new_owned();
        self.compress(image, &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Compress the `image` into the slice `dest`.
    ///
    /// Returns the size of the compressed image. If the compressed image does not fit into `dest`,
    /// this method returns an error. Use [`buf_len`](Compressor::buf_len) to determine buffer size
    /// that is guaranteed to be large enough for the compressed image.
    pub fn compress_to_slice(&mut self, image: Image<&[u8]>, dest: &mut [u8]) -> Result<usize> {
        let mut buf = OutputBuf::borrowed(dest);
        self.compress(image, &mut buf)?;
        Ok(buf.len())
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
