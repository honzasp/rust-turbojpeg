use std::convert::TryInto as _;
use crate::{Image, raw};
use crate::common::{PixelFormat, Subsamp, Colorspace, Result, Error, get_error};

/// Decompresses JPEG data into raw pixels.
#[derive(Debug)]
#[doc(alias = "tjhandle")]
pub struct Decompressor {
    handle: raw::tjhandle,
}

unsafe impl Send for Decompressor {}

/// JPEG header that describes the compressed image.
///
/// The header can be obtained without decompressing the image by calling
/// [`Decompressor::read_header()`] or [`read_header()`][crate::read_header].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DecompressHeader {
    /// Width of the image in pixels (number of columns).
    pub width: usize,
    /// Height of the image in pixels (number of rows).
    pub height: usize,
    /// Chrominance subsampling that is used in the compressed image.
    pub subsamp: Subsamp,
    /// Colorspace of the compressed image.
    pub colorspace: Colorspace,
}

impl Decompressor {
    /// Create a new decompressor instance.
    #[doc(alias = "tjInitDecompress")]
    pub fn new() -> Result<Decompressor> {
        unsafe {
            let handle = raw::tjInitDecompress();
            if !handle.is_null() {
                Ok(Decompressor { handle })
            } else {
                Err(get_error(handle))
            }
        }
    }

    /// Read the JPEG header without decompressing the image.
    ///
    /// # Example
    ///
    /// ```
    /// // read JPEG data from file
    /// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
    ///
    /// // initialize a decompressor
    /// let mut decompressor = turbojpeg::Decompressor::new()?;
    ///
    /// // read the JPEG header
    /// let header = decompressor.read_header(&jpeg_data)?;
    /// assert_eq!((header.width, header.height), (384, 256));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn read_header(&mut self, jpeg_data: &[u8]) -> Result<DecompressHeader> {
        let jpeg_data_len = jpeg_data.len().try_into()
            .map_err(|_| Error::IntegerOverflow("jpeg_data.len()"))?;
        let mut width = 0;
        let mut height = 0;
        let mut subsamp = 0;
        let mut colorspace = 0;
        let res = unsafe {
            raw::tjDecompressHeader3(
                self.handle,
                jpeg_data.as_ptr(), jpeg_data_len,
                &mut width, &mut height, &mut subsamp, &mut colorspace,
            )
        };

        if res == 0 {
            let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
            let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
            let subsamp = Subsamp::from_u32(subsamp as u32)?;
            let colorspace = Colorspace::from_u32(colorspace as u32)?;
            Ok(DecompressHeader { width, height, subsamp, colorspace })
        } else {
            Err(unsafe { get_error(self.handle) })
        }
    }

    /// Decompress a JPEG image in `jpeg_data` into `output`.
    ///
    /// The decompressed image is stored in the pixel data of the given `output` image, which must
    /// be fully initialized by the caller. Use [`read_header()`](Decompressor::read_header) to
    /// determine the image size before calling this method.
    ///
    /// # Example
    ///
    /// ```
    /// // read JPEG data from file
    /// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
    ///
    /// // initialize a decompressor
    /// let mut decompressor = turbojpeg::Decompressor::new()?;
    ///
    /// // read the JPEG header
    /// let header = decompressor.read_header(&jpeg_data)?;
    ///
    /// // initialize the image (Image<Vec<u8>>)
    /// let mut image = turbojpeg::Image {
    ///     pixels: vec![0; 4 * header.width * header.height],
    ///     width: header.width,
    ///     pitch: 4 * header.width, // size of one image row in memory
    ///     height: header.height,
    ///     format: turbojpeg::PixelFormat::RGBA,
    /// };
    ///
    /// // decompress the JPEG into the image
    /// // (we use as_deref_mut() to convert from &mut Image<Vec<u8>> into Image<&mut [u8]>)
    /// decompressor.decompress(&jpeg_data, image.as_deref_mut())?;
    /// assert_eq!(&image.pixels[0..4], &[122, 118, 89, 255]);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "tjDecompress2")]
    pub fn decompress(&mut self, jpeg_data: &[u8], output: Image<&mut [u8]>) -> Result<()> {
        output.assert_valid(output.pixels.len());

        let Image { pixels, width, pitch, height, format } = output;
        let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let pitch = pitch.try_into().map_err(|_| Error::IntegerOverflow("pitch"))?;
        let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
        let jpeg_data_len = jpeg_data.len().try_into()
            .map_err(|_| Error::IntegerOverflow("jpeg_data.len()"))?;

        let res = unsafe {
            raw::tjDecompress2(
                self.handle,
                jpeg_data.as_ptr(), jpeg_data_len,
                pixels.as_mut_ptr(), width, pitch, height, format as i32,
                0,
            )
        };

        if res == 0 {
            Ok(())
        } else {
            Err(unsafe { get_error(self.handle) })
        }
    }
}

impl Drop for Decompressor {
    fn drop(&mut self) {
        unsafe { raw::tjDestroy(self.handle); }
    }
}

/// Decompress a JPEG image.
///
/// Returns a newly allocated image with the given pixel `format`. If you have specific
/// requirements regarding memory layout or allocations, please see [`Decompressor`].
///
/// # Example
///
/// ```
/// // read JPEG data from file
/// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
///
/// // decompress the JPEG into RGB image
/// let image = turbojpeg::decompress(&jpeg_data, turbojpeg::PixelFormat::RGB)?;
/// assert_eq!(image.format, turbojpeg::PixelFormat::RGB);
/// assert_eq!((image.width, image.height), (384, 256));
/// assert_eq!(image.pixels.len(), 384 * 256 * 3);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn decompress(jpeg_data: &[u8], format: PixelFormat) -> Result<Image<Vec<u8>>> {
    let mut decompressor = Decompressor::new()?;
    let header = decompressor.read_header(jpeg_data)?;

    let pitch = header.width * format.size();
    let mut image = Image {
        pixels: vec![0; header.height * pitch],
        width: header.width,
        pitch,
        height: header.height,
        format,
    };
    decompressor.decompress(jpeg_data, image.as_deref_mut())?;

    Ok(image)
}

/// Read the JPEG header without decompressing the image.
///
/// # Example
///
/// ```
/// // read JPEG data from file
/// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
///
/// // read the JPEG header
/// let header = turbojpeg::read_header(&jpeg_data)?;
/// assert_eq!((header.width, header.height), (384, 256));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn read_header(jpeg_data: &[u8]) -> Result<DecompressHeader> {
    let mut decompressor = Decompressor::new()?;
    decompressor.read_header(jpeg_data)
}
