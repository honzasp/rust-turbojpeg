use std::convert::TryInto as _;
use std::fmt;
use crate::{Image, YuvImage, raw};
use crate::common::{PixelFormat, Subsamp, Colorspace, Result, Error};
use crate::handle::Handle;

/// Decompresses JPEG data into raw pixels.
#[derive(Debug)]
#[doc(alias = "tjhandle")]
pub struct Decompressor {
    handle: Handle,
    scaling_factor: ScalingFactor,
}

unsafe impl Send for Decompressor {}

/// JPEG header that describes the compressed image.
///
/// The header can be obtained without decompressing the image by calling
/// [`Decompressor::read_header()`] or [`read_header()`][crate::read_header].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DecompressHeader {
    /// Width of the image in pixels (number of columns).
    pub width: usize,
    /// Height of the image in pixels (number of rows).
    pub height: usize,
    /// Chrominance subsampling that is used in the compressed image.
    pub subsamp: Subsamp,
    /// Colorspace of the compressed image.
    pub colorspace: Colorspace,
    /// Is the image lossless JPEG?
    pub is_lossless: bool,
}

/// Fractional scaling factor.
///
/// TurboJPEG can efficiently scale a JPEG image when decompressing. The scaling is implemented in
/// the DCT algorithm, so scaling factors are limited to multiples of 1/8. Use
/// [`Decompressor::supported_scaling_factors()`] to get the list of all scaling factors supported
/// by the decompressor.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[doc(alias = "tjscalingfactor")]
pub struct ScalingFactor {
    num: usize,
    denom: usize,
}

impl ScalingFactor {
    /// 1x scaling (no effect).
    pub const ONE: Self = Self { num: 1, denom: 1 };
    /// 1/2x scaling factor.
    pub const ONE_HALF: Self = Self { num: 1, denom: 2 };
    /// 1/4x scaling factor.
    pub const ONE_QUARTER: Self = Self { num: 1, denom: 4 };
    /// 1/8x scaling factor.
    pub const ONE_EIGHTH: Self = Self { num: 1, denom: 8 };
    /// 2x scaling factor.
    pub const TWO: Self = Self { num: 2, denom: 1 };

    /// Create a scaling factor from `num`-erator and `denom`-inator.
    ///
    /// We will simplify the fraction, so the numerator and denominator of the resulting fraction
    /// might be different from what you pass.
    ///
    /// # Example
    ///
    /// ```
    /// let s = turbojpeg::ScalingFactor::new(12, 8);
    /// assert_eq!(s.num(), 3);
    /// assert_eq!(s.denom(), 2);
    /// ```
    pub fn new(num: usize, denom: usize) -> Self {
        let gcd = gcd::binary_usize(num, denom);
        Self { num: num / gcd, denom: denom / gcd }
    }

    /// Get the numerator (the "3" in "3/4").
    pub fn num(&self) -> usize {
        self.num
    }

    /// Get the denominator (the "4" in "3/4").
    pub fn denom(&self) -> usize {
        self.denom
    }

    /// Compute the value of `dimension` scaled by this scaling factor.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(turbojpeg::ScalingFactor::ONE_QUARTER.scale(400), 100);
    /// assert_eq!(turbojpeg::ScalingFactor::ONE_QUARTER.scale(5), 2);
    /// assert_eq!(turbojpeg::ScalingFactor::new(7, 8).scale(20), 18);
    /// ```
    #[doc(alias = "TJSCALED")]
    pub fn scale(&self, dimension: usize) -> usize {
        (dimension * self.num + self.denom - 1) / self.denom
    }
}

impl fmt::Display for ScalingFactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.denom)
    }
}

impl DecompressHeader {
    /// Scale the image size (width, height) in the header by the scaling factor.
    ///
    /// # Example
    ///
    /// ```
    /// // read JPEG header from file
    /// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
    /// let header = turbojpeg::read_header(&jpeg_data)?;
    /// assert_eq!((header.width, header.height), (384, 256));
    ///
    /// // scale the header
    /// let scale_factor = turbojpeg::ScalingFactor::ONE_HALF;
    /// let scaled_header = header.scaled(scale_factor);
    /// assert_eq!((scaled_header.width, scaled_header.height), (192, 128));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJSCALED")]
    pub fn scaled(&self, factor: ScalingFactor) -> Self {
        Self {
            width: factor.scale(self.width),
            height: factor.scale(self.height),
            .. *self
        }
    }
}

impl Decompressor {
    /// Create a new decompressor instance.
    #[doc(alias = "tj3Init")]
    pub fn new() -> Result<Decompressor> {
        let handle = Handle::new(raw::TJINIT_TJINIT_DECOMPRESS)?;
        Ok(Self { handle, scaling_factor: ScalingFactor::ONE })
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
    #[doc(alias = "tj3DecompressHeader")]
    pub fn read_header(&mut self, jpeg_data: &[u8]) -> Result<DecompressHeader> {
        let jpeg_data_len = jpeg_data.len().try_into()
            .map_err(|_| Error::IntegerOverflow("jpeg_data.len()"))?;
        let res = unsafe {
            raw::tj3DecompressHeader(self.handle.as_ptr(), jpeg_data.as_ptr(), jpeg_data_len)
        };
        if res != 0 {
            return Err(self.handle.get_error())
        }

        let width = self.handle.get(raw::TJPARAM_TJPARAM_JPEGWIDTH)
            .try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let height = self.handle.get(raw::TJPARAM_TJPARAM_JPEGHEIGHT)
            .try_into().map_err(|_| Error::IntegerOverflow("height"))?;
        let subsamp = Subsamp::from_int(self.handle.get(raw::TJPARAM_TJPARAM_SUBSAMP))?;
        let colorspace = Colorspace::from_int(self.handle.get(raw::TJPARAM_TJPARAM_COLORSPACE))?;
        let is_lossless = self.handle.get(raw::TJPARAM_TJPARAM_LOSSLESS) != 0;
        Ok(DecompressHeader { width, height, subsamp, colorspace, is_lossless })
    }

    /// Set scaling factor for subsequent decompression operations.
    ///
    /// Only the scaling factors returned by
    /// [`supported_scaling_factors()`][Self::supported_scaling_factors()] are supported, and a
    /// scaling factor can only be used when decompressing lossy JPEG images.
    ///
    /// # Example
    ///
    /// ```
    /// // read JPEG data from file
    /// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
    ///
    /// // initialize a decompressor with the scaling factor
    /// let mut decompressor = turbojpeg::Decompressor::new()?;
    /// let scaling = turbojpeg::ScalingFactor::ONE_HALF;
    /// decompressor.set_scaling_factor(scaling);
    ///
    /// // read the JPEG header and downscale the width and height
    /// let scaled_header = decompressor.read_header(&jpeg_data)?.scaled(scaling);
    ///
    /// // initialize the image (Image<Vec<u8>>)
    /// let mut image = turbojpeg::Image {
    ///     pixels: vec![0; 4 * scaled_header.width * scaled_header.height],
    ///     width: scaled_header.width,
    ///     pitch: 4 * scaled_header.width, // size of one image row in memory
    ///     height: scaled_header.height,
    ///     format: turbojpeg::PixelFormat::RGBA,
    /// };
    ///
    /// // decompress the JPEG into the image
    /// // (we use as_deref_mut() to convert from &mut Image<Vec<u8>> into Image<&mut [u8]>)
    /// decompressor.decompress(&jpeg_data, image.as_deref_mut())?;
    /// assert_eq!(&image.pixels[0..5], &[125, 121, 92, 255, 127]);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "tj3SetScalingFactor")]
    pub fn set_scaling_factor(&mut self, scaling_factor: ScalingFactor) -> Result<()> {
        let num: libc::c_int = scaling_factor.num.try_into()
            .map_err(|_| Error::IntegerOverflow("num"))?;
        let denom: libc::c_int = scaling_factor.denom.try_into()
            .map_err(|_| Error::IntegerOverflow("denom"))?;
        self.handle.set_scaling_factor(raw::tjscalingfactor { num, denom })?;
        self.scaling_factor = scaling_factor;
        Ok(())
    }

    /// Get the scaling factor set by [`set_scaling_factor()`][Self::set_scaling_factor()].
    pub fn scaling_factor(&self) -> ScalingFactor {
        self.scaling_factor
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
    #[doc(alias = "tj3Decompress8")]
    pub fn decompress(&mut self, jpeg_data: &[u8], output: Image<&mut [u8]>) -> Result<()> {
        output.assert_valid(output.pixels.len());
        let Image { pixels, width, pitch, height, format } = output;
        let width: libc::c_int = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let pitch: libc::c_int = pitch.try_into().map_err(|_| Error::IntegerOverflow("pitch"))?;
        let height: libc::c_int = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
        let jpeg_data_len: raw::size_t = jpeg_data.len().try_into()
            .map_err(|_| Error::IntegerOverflow("jpeg_data.len()"))?;

        self.check_output_size(jpeg_data, width, height)?;

        let res = unsafe {
            raw::tj3Decompress8(
                self.handle.as_ptr(),
                jpeg_data.as_ptr(), jpeg_data_len,
                pixels.as_mut_ptr(), pitch, format as i32,
            )
        };
        if res != 0 {
            return Err(self.handle.get_error())
        }

        Ok(())
    }

    /// Decompress a JPEG image in `jpeg_data` into `output` as YUV without changing color space.
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
    /// // calculate YUV pixels length
    /// let align = 4;
    /// let yuv_pixels_len = turbojpeg::yuv_pixels_len(header.width, align, header.height, header.subsamp);
    ///
    /// // initialize the image (YuvImage<Vec<u8>>)
    /// let mut image = turbojpeg::YuvImage {
    ///     pixels: vec![0; yuv_pixels_len.unwrap()],
    ///     width: header.width,
    ///     align,
    ///     height: header.height,
    ///     subsamp: header.subsamp,
    /// };
    ///
    /// // decompress the JPEG into the image
    /// // (we use as_deref_mut() to convert from &mut YuvImage<Vec<u8>> into YuvImage<&mut [u8]>)
    /// decompressor.decompress_to_yuv(&jpeg_data, image.as_deref_mut())?;
    /// assert_eq!(&image.pixels[0..4], &[116, 117, 118, 119]);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "tj3DecompressToYUV8")]
    pub fn decompress_to_yuv(&mut self, jpeg_data: &[u8], output: YuvImage<&mut [u8]>) -> Result<()> {
        output.assert_valid(output.pixels.len());
        let YuvImage { pixels, width, align, height, subsamp: _ } = output;
        let width: libc::c_int = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let align: libc::c_int = align.try_into().map_err(|_| Error::IntegerOverflow("align"))?;
        let height: libc::c_int = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
        let jpeg_data_len: raw::size_t = jpeg_data.len().try_into()
            .map_err(|_| Error::IntegerOverflow("jpeg_data.len()"))?;

        self.check_output_size(jpeg_data, width, height)?;

        let res = unsafe {
            raw::tj3DecompressToYUV8(
                self.handle.as_ptr(),
                jpeg_data.as_ptr(), jpeg_data_len,
                pixels.as_mut_ptr(), align,
            )
        };
        if res != 0 {
            return Err(self.handle.get_error())
        }

        Ok(())
    }

    fn check_output_size(&mut self, jpeg_data: &[u8], width: libc::c_int, height: libc::c_int) -> Result<()> {
        let header = self.read_header(jpeg_data)?;

        if header.is_lossless && self.scaling_factor != ScalingFactor::ONE {
            return Err(Error::CannotScaleLossless)
        }
        let scaled_width = self.scaling_factor.scale(header.width);
        let scaled_height = self.scaling_factor.scale(header.height);

        if width < scaled_width as i32 || height < scaled_height as i32 {
            return Err(Error::OutputTooSmall(scaled_width as i32, scaled_height as i32))
        }

        Ok(())
    }

    /// Get the list of scaling factors supported for decompression.
    ///
    /// At the time of this writing, TurboJPEG supports all multiples of 1/8 between 1/8 and 2 as
    /// scaling factors, but it's unclear whether this is guaranteed to continue to be the case in
    /// all future versions.
    ///
    /// # Example
    ///
    /// ```
    /// let factors = turbojpeg::Decompressor::supported_scaling_factors();
    /// for num in 1..16 {
    ///     let multiple_of_8 = turbojpeg::ScalingFactor::new(num, 8);
    ///     assert!(factors.iter().find(|&f| *f == multiple_of_8).is_some());
    /// }
    /// ```
    #[doc(alias = "tj3GetScalingFactors")]
    pub fn supported_scaling_factors() -> Vec<ScalingFactor> {
        let mut count: libc::c_int = 0;
        let ptr: *const raw::tjscalingfactor = unsafe {
            raw::tj3GetScalingFactors(&mut count as *mut _)
        };
        let count: usize = count.try_into()
            .expect("tj3GetScalingFactors() returned a number that cannot be converted to usize");

        let mut list = Vec::with_capacity(count);
        for i in 0..count {
            let factor = unsafe { ptr.add(i).read() };
            let num: usize = factor.num.try_into()
                .expect("Numerator of a tjscalingfactor cannot be converted to usize");
            let denom: usize = factor.denom.try_into()
                .expect("Denominator of a tjscalingfactor cannot be converted to usize");
            list.push(ScalingFactor { num, denom });
        }
        list
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

/// Decompress a JPEG image to YUV.
///
/// Returns a newly allocated YUV image with row alignment of 4. If you have specific requirements
/// regarding memory layout or allocations, please see [`Decompressor`].
///
/// # Example
///
/// ```
/// // read JPEG data from file
/// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
///
/// // decompress the JPEG into YUV image
/// let image = turbojpeg::decompress_to_yuv(&jpeg_data)?;
/// assert_eq!((image.width, image.height), (384, 256));
/// assert_eq!(image.pixels.len(), 294912);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn decompress_to_yuv(jpeg_data: &[u8]) -> Result<YuvImage<Vec<u8>>> {
    let mut decompressor = Decompressor::new()?;
    let header = decompressor.read_header(jpeg_data)?;
    let align = 4;
    let yuv_pixels_len = yuv_pixels_len(
        header.width,
        align,
        header.height,
        header.subsamp,
    )?;

    let mut yuv_image = YuvImage {
        pixels: vec![0; yuv_pixels_len],
        width: header.width,
        align,
        height: header.height,
        subsamp: header.subsamp,
    };
    decompressor.decompress_to_yuv(jpeg_data, yuv_image.as_deref_mut())?;

    Ok(yuv_image)
}

/// Determine size in bytes of a YUV image.
///
/// Calculates the size for [`YuvImage::pixels`] based on the image width, height, chrominance
/// subsampling and row alignment.
///
/// Returns an error on integer overflow. You can just `.unwrap()` the result if you don't care
/// about this edge case.
/// 
/// # Example
///
/// ```
/// // read JPEG data from file
/// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
///
/// // read the JPEG header
/// let header = turbojpeg::read_header(&jpeg_data)?;
/// // get YUV pixels length
/// let align = 4;
/// let yuv_pixels_len = turbojpeg::yuv_pixels_len(header.width, align, header.height, header.subsamp);
/// assert_eq!(yuv_pixels_len.unwrap(), 294912);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[doc(alias = "tj3YUVBufSize")]
pub fn yuv_pixels_len(width: usize, align: usize, height: usize, subsamp: Subsamp) -> Result<usize> {
    let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
    let align = align.try_into().map_err(|_| Error::IntegerOverflow("align"))?;
    let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
    let len = unsafe { raw::tj3YUVBufSize(width, align, height, subsamp as libc::c_int) };
    let len = len.try_into().map_err(|_| Error::IntegerOverflow("yuv size"))?;
    Ok(len)
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
