use std::convert::TryInto as _;
use crate::{Image, raw};
use crate::buf::{OwnedBuf, OutputBuf};
use crate::common::{Subsamp, Result, Error, get_error};

/// Compresses raw pixel data into JPEG.
#[derive(Debug)]
#[doc(alias = "tjhandle")]
pub struct Compressor {
    handle: raw::tjhandle,
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
            let handle = raw::tjInitCompress();
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

    /// Compresses the `image` into `output` buffer.
    ///
    /// This is the main compression method, which gives you full control of the output buffer. If
    /// you don't need this level of control, you can use one of the convenience wrappers below.
    ///
    /// # Example
    ///
    /// ```
    /// // create an image (a Mandelbrot set visualization)
    /// let image = turbojpeg::Image::mandelbrot(500, 500, turbojpeg::PixelFormat::RGB);
    ///
    /// // initialize the compressor
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_quality(70);
    /// compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);
    ///
    /// // initialize the output buffer
    /// let mut output_buf = turbojpeg::OutputBuf::new_owned();
    ///
    /// // compress the image into JPEG
    /// // (we use as_deref() to convert from &Image<Vec<u8>> to Image<&[u8]>)
    /// compressor.compress(image.as_deref(), &mut output_buf)?;
    ///
    /// // write the JPEG to disk
    /// std::fs::write(std::env::temp_dir().join("mandelbrot.jpg"), &output_buf)?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "tjCompress2")]
    #[doc(alias = "tjCompress")]
    pub fn compress(&mut self, image: Image<&[u8]>, output: &mut OutputBuf) -> Result<()> {
        image.assert_valid(image.pixels.len());

        let Image { pixels, width, pitch, height, format } = image;
        let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let pitch = pitch.try_into().map_err(|_| Error::IntegerOverflow("pitch"))?;
        let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;

        let mut output_len = output.len as libc::c_ulong;
        let res = unsafe {
            raw::tjCompress2(
                self.handle,
                pixels.as_ptr(), width, pitch, height, format as libc::c_int,
                &mut output.ptr, &mut output_len,
                self.subsamp as libc::c_int, self.quality,
                if output.is_owned { 0 } else { raw::TJFLAG_NOREALLOC } as libc::c_int,
            )
        };
        output.len = output_len as usize;

        if res != 0 {
            Err(unsafe { get_error(self.handle) })
        } else if output.ptr.is_null() {
            output.len = 0;
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
    /// extra allocation and copying, consider using [`compress_to_owned()`][Self::compress_to_owned]
    /// instead.
    pub fn compress_to_vec(&mut self, image: Image<&[u8]>) -> Result<Vec<u8>> {
        let mut buf = OutputBuf::new_owned();
        self.compress(image, &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Compress the `image` into the slice `output`.
    ///
    /// Returns the size of the compressed JPEG data. If the compressed image does not fit into
    /// `dest`, this method returns an error. Use [`buf_len()`](Compressor::buf_len) to determine
    /// buffer size that is guaranteed to be large enough for the compressed image.
    pub fn compress_to_slice(&mut self, image: Image<&[u8]>, output: &mut [u8]) -> Result<usize> {
        let mut buf = OutputBuf::borrowed(output);
        self.compress(image, &mut buf)?;
        Ok(buf.len())
    }

    /// Compute the maximum size of a compressed image.
    ///
    /// This depends on image `width` and `height`, and also on the current setting of chrominance
    /// subsampling (see [`set_subsamp()`](Compressor::set_subsamp)).
    ///
    /// You can also use [`compressed_buf_len()`] directly.
    #[doc(alias = "tjBufSize")]
    pub fn buf_len(&self, width: usize, height: usize) -> Result<usize> {
        super::compressed_buf_len(width, height, self.subsamp)
    }
}

impl Drop for Compressor {
    fn drop(&mut self) {
        unsafe { raw::tjDestroy(self.handle); }
    }
}

/// Compress a JPEG image.
/// 
/// Uses the given quality and chrominance subsampling option and returns the JPEG data in a buffer
/// owned by TurboJPEG. If this function does not fit your needs, please see [`Compressor`].
///
/// # Example
///
/// ```
/// // create an image (a Mandelbrot set visualization)
/// let image = turbojpeg::Image::mandelbrot(500, 500, turbojpeg::PixelFormat::RGB);
///
/// // compress the image into JPEG with quality 75 and in grayscale
/// // (we use as_deref() to convert from &Image<Vec<u8>> to Image<&[u8]>)
/// let jpeg_data = turbojpeg::compress(image.as_deref(), 75, turbojpeg::Subsamp::Sub2x2)?;
///
/// // write the JPEG to disk
/// std::fs::write(std::env::temp_dir().join("mandelbrot.jpg"), &jpeg_data)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn compress(image: Image<&[u8]>, quality: i32, subsamp: Subsamp) -> Result<OwnedBuf> {
    let mut compressor = Compressor::new()?;
    compressor.set_quality(quality);
    compressor.set_subsamp(subsamp);
    compressor.compress_to_owned(image)
}

/// Compute the maximum size of a compressed image.
///
/// This depends on image `width` and `height` and also on the chrominance subsampling method.
///
/// Returns an error on integer overflow. You can just `.unwrap()` the result if you don't care
/// about this edge case.
#[doc(alias = "tjBufSize")]
pub fn compressed_buf_len(width: usize, height: usize, subsamp: Subsamp) -> Result<usize> {
    let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
    let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
    let len = unsafe { raw::tjBufSize(width, height, subsamp as libc::c_int) };
    let len = len.try_into().map_err(|_| Error::IntegerOverflow("buf len"))?;
    Ok(len)
}
