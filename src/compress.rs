use std::convert::TryInto as _;
use crate::{Image, raw};
use crate::buf::{OwnedBuf, OutputBuf};
use crate::common::{Subsamp, Result, Error};
use crate::handle::Handle;

/// Compresses raw pixel data into JPEG.
#[derive(Debug)]
#[doc(alias = "tjhandle")]
pub struct Compressor {
    handle: Handle,
    subsamp: Subsamp,
}

static DEFAULT_QUALITY: i32 = 95;
static DEFAULT_SUBSAMP: Subsamp = Subsamp::None;

unsafe impl Send for Compressor {}

impl Compressor {
    /// Create a new compressor instance.
    #[doc(alias = "tj3Init")]
    pub fn new() -> Result<Compressor> {
        let mut handle = Handle::new(raw::TJINIT_TJINIT_COMPRESS)?;
        handle.set(raw::TJPARAM_TJPARAM_QUALITY, DEFAULT_QUALITY as libc::c_int)?;
        handle.set(raw::TJPARAM_TJPARAM_SUBSAMP, DEFAULT_SUBSAMP as i32 as libc::c_int)?;
        Ok(Compressor { handle, subsamp: DEFAULT_SUBSAMP })
    }

    /// Set the quality of the compressed JPEG images.
    ///
    /// The quality ranges from 1 (worst) to 100 (best).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_quality(95)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// An error is returned if `quality` is invalid:
    ///
    /// ```
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// assert!(compressor.set_quality(120).is_err());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_QUALITY")]
    pub fn set_quality(&mut self, quality: i32) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_QUALITY, quality as libc::c_int)
    }

    /// Set the level of chrominance subsampling of the compressed JPEG images.
    ///
    /// Chrominance subsampling can reduce the compressed image size without noticeable loss of
    /// quality (see [`Subsamp`] for more).
    #[doc(alias = "TJPARAM_SUBSAMP")]
    pub fn set_subsamp(&mut self, subsamp: Subsamp) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_SUBSAMP, subsamp as i32 as libc::c_int)
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
    #[doc(alias = "tj3Compress8")]
    pub fn compress(&mut self, image: Image<&[u8]>, output: &mut OutputBuf) -> Result<()> {
        image.assert_valid(image.pixels.len());

        let Image { pixels, width, pitch, height, format } = image;
        let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let pitch = pitch.try_into().map_err(|_| Error::IntegerOverflow("pitch"))?;
        let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;

        self.handle.set(
            raw::TJPARAM_TJPARAM_NOREALLOC,
            if output.is_owned { 0 } else { 1 } as libc::c_int,
        )?;
        let mut output_len = output.len as raw::size_t;
        let res = unsafe {
            raw::tj3Compress8(
                self.handle.as_ptr(),
                pixels.as_ptr(), width, pitch, height, format as libc::c_int,
                &mut output.ptr, &mut output_len,
            )
        };
        output.len = output_len as usize;
        if res != 0 {
            return Err(self.handle.get_error())
        } else if output.ptr.is_null() {
            output.len = 0;
            return Err(Error::Null)
        }

        Ok(())
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
    #[doc(alias = "tj3JPEGBufSize")]
    pub fn buf_len(&self, width: usize, height: usize) -> Result<usize> {
        compressed_buf_len(width, height, self.subsamp)
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
    compressor.set_quality(quality)?;
    compressor.set_subsamp(subsamp)?;
    compressor.compress_to_owned(image)
}

/// Compute the maximum size of a compressed image.
///
/// This depends on image `width` and `height` and also on the chrominance subsampling method.
///
/// Returns an error on integer overflow. You can just `.unwrap()` the result if you don't care
/// about this edge case.
#[doc(alias = "tj3JPEGBufSize")]
pub fn compressed_buf_len(width: usize, height: usize, subsamp: Subsamp) -> Result<usize> {
    let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
    let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
    let len = unsafe { raw::tj3JPEGBufSize(width, height, subsamp as libc::c_int) };
    let len = len.try_into().map_err(|_| Error::IntegerOverflow("buf len"))?;
    Ok(len)
}
