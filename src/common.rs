/// Pixel format determines the layout of pixels in memory.
#[doc(alias = "TJPF")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum PixelFormat {
    /// RGB pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 3-byte pixels in the order
    /// R, G, B from lowest to highest byte address within each pixel.
    #[doc(alias = "TJPF_RGB")]
    RGB = raw::TJPF_TJPF_RGB,

    /// BGR pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 3-byte pixels in the order
    /// B, G, R from lowest to highest byte address within each pixel.
    #[doc(alias = "TJPF_BGR")]
    BGR = raw::TJPF_TJPF_BGR,

    /// RGBX pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// R, G, B from lowest to highest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_RGBX")]
    RGBX = raw::TJPF_TJPF_RGBX,

    /// BGRX pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// B, G, R from lowest to highest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_BGRX")]
    BGRX = raw::TJPF_TJPF_BGRX,

    /// XBGR pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// R, G, B from highest to lowest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_XBGR")]
    XBGR = raw::TJPF_TJPF_XBGR,

    /// XRGB pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// B, G, R from highest to lowest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_XRGB")]
    XRGB = raw::TJPF_TJPF_XRGB,

    /// Grayscale pixel format.
    ///
    /// Each 1-byte pixel represents a luminance (brightness) level from 0 to 255.
    #[doc(alias = "TJPF_GRAY")]
    GRAY = raw::TJPF_TJPF_GRAY,

    /// RGBA pixel format.
    ///
    /// This is the same as [`PixelFormat::RGBX`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_RGBA")]
    RGBA = raw::TJPF_TJPF_RGBA,

    /// BGRA pixel format.
    ///
    /// This is the same as [`PixelFormat::BGRX`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_BGRA")]
    BGRA = raw::TJPF_TJPF_BGRA,

    /// ABGR pixel format.
    ///
    /// This is the same as [`PixelFormat::XBGR`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_ABGR")]
    ABGR = raw::TJPF_TJPF_ABGR,

    /// ARGB pixel format.
    ///
    /// This is the same as [`PixelFormat::ARGB`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_ARGB")]
    ARGB = raw::TJPF_TJPF_ARGB,

    /// CMYK pixel format.
    ///
    /// Unlike RGB, which is an additive color model used primarily for display, CMYK
    /// (Cyan/Magenta/Yellow/Key) is a subtractive color model used primarily for printing. In the
    /// CMYK color model, the value of each color component typically corresponds to an amount of
    /// cyan, magenta, yellow, or black ink that is applied to a white background. In order to
    /// convert between CMYK and RGB, it is necessary to use a color management system (CMS). A CMS
    /// will attempt to map colors within the printer's gamut to perceptually similar colors in the
    /// display's gamut and vice versa, but the mapping is typically not 1:1 or reversible, nor can
    /// it be defined with a simple formula. Thus, such a conversion is out of scope for a codec
    /// library. However, the TurboJPEG API allows for compressing CMYK pixels into a YCCK JPEG
    /// image (see TJCS_YCCK) and decompressing YCCK JPEG images into CMYK pixels.
    #[doc(alias = "TJPF_CMYK")]
    CMYK = raw::TJPF_TJPF_CMYK,
}

impl PixelFormat {
    /// The size of a pixel in bytes.
    pub fn size(&self) -> usize {
        match self {
            PixelFormat::RGB => 3,
            PixelFormat::BGR => 3,
            PixelFormat::RGBX => 4,
            PixelFormat::BGRX => 4,
            PixelFormat::XBGR => 4,
            PixelFormat::XRGB => 4,
            PixelFormat::GRAY => 1,
            PixelFormat::RGBA => 4,
            PixelFormat::BGRA => 4,
            PixelFormat::ABGR => 4,
            PixelFormat::ARGB => 4,
            PixelFormat::CMYK => 4,
        }
    }
}


/// Chrominance subsampling options.
///
/// When pixels are converted from RGB to YCbCr or from CMYK to YCCK as part of the JPEG
/// compression process, some of the Cb and Cr (chrominance) components can be discarded or
/// averaged together to produce a smaller image with little perceptible loss of image clarity (the
/// human eye is more sensitive to small changes in brightness than to small changes in color).
/// This is called "chrominance subsampling".
#[doc(alias = "TJSAMP")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
#[non_exhaustive]
pub enum Subsamp {
    /// No chrominance subsampling (4:4:4);
    ///
    /// The JPEG or YUV image will contain one chrominance component for every pixel in the source
    /// image.
    #[doc(alias = "TJSAMP_444")]
    None = raw::TJSAMP_TJSAMP_444,

    /// 2x1 chrominance subsampling (4:2:2).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 2x1 block of pixels
    /// in the source image.
    #[doc(alias = "TJSAMP_422")]
    Sub2x1 = raw::TJSAMP_TJSAMP_422,

    /// 2x2 chrominance subsampling (4:2:0).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 2x2 block of pixels
    /// in the source image.
    #[doc(alias = "TJSAMP_420")]
    Sub2x2 = raw::TJSAMP_TJSAMP_420,

    /// Grayscale.
    ///
    /// The JPEG or YUV image will contain no chrominance components.
    #[doc(alias = "TJSAMP_GRAY")]
    Gray = raw::TJSAMP_TJSAMP_GRAY,

    /// 1x2 chrominance subsampling (4:4:0).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 1x2 block of pixels
    /// in the source image.
    ///
    /// # Note
    ///
    /// 4:4:0 subsampling is not fully accelerated in libjpeg-turbo.
    #[doc(alias = "TJSAMP_440")]
    Sub1x2 = raw::TJSAMP_TJSAMP_440,

    /// 4x1 chrominance subsampling (4:1:1).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 4x1 block of pixels
    /// in the source image. JPEG images compressed with 4:1:1 subsampling will be almost exactly
    /// the same size as those compressed with 4:2:0 subsampling, and in the aggregate, both
    /// subsampling methods produce approximately the same perceptual quality. However, 4:1:1 is
    /// better able to reproduce sharp horizontal features.
    ///
    /// # Note
    ///
    /// 4:1:1 subsampling is not fully accelerated in libjpeg-turbo.
    #[doc(alias = "TJSAMP_411")]
    Sub4x1 = raw::TJSAMP_TJSAMP_411,

    /// 1x4 chrominance subsampling (4:4:1).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 1x4 block of pixels
    /// in the source image. JPEG images compressed with 4:4:1 subsampling will be almost exactly
    /// the same size as those compressed with 4:2:0 subsampling, and in the aggregate, both
    /// subsampling methods produce approximately the same perceptual quality.  However, 4:4:1 is
    /// better able to reproduce sharp vertical features.
    ///
    /// # Note
    ///
    /// 4:4:1 subsampling is not fully accelerated in libjpeg-turbo.
    #[doc(alias = "TJSAMP_441")]
    Sub1x4 = raw::TJSAMP_TJSAMP_441,

    /// Unknown subsampling.
    ///
    /// The JPEG image uses an unusual type of chrominance subsampling. Such images can be
    /// decompressed into packed-pixel images, but they cannot be
    ///
    /// - decompressed into planar YUV images,
    /// - losslessly transformed if [`Transform::crop`][crate::Transform::crop] is specified and
    /// [`Transform::gray`][crate::Transform::gray] is not specified, or
    /// - partially decompressed using a cropping region.
    #[doc(alias = "TJSAMP_UNKNOWN")]
    Unknown = raw::TJSAMP_TJSAMP_UNKNOWN,
}

impl Subsamp {
    pub(crate) fn from_int(subsamp: libc::c_int) -> Result<Self> {
        Ok(match subsamp {
            raw::TJSAMP_TJSAMP_444 => Self::None,
            raw::TJSAMP_TJSAMP_422 => Self::Sub2x1,
            raw::TJSAMP_TJSAMP_420 => Self::Sub2x2,
            raw::TJSAMP_TJSAMP_GRAY => Self::Gray,
            raw::TJSAMP_TJSAMP_440 => Self::Sub1x2,
            raw::TJSAMP_TJSAMP_411 => Self::Sub4x1,
            raw::TJSAMP_TJSAMP_UNKNOWN => Self::Unknown,
            other => return Err(Error::BadSubsamp(other)),
        })
    }

    /// Get the width of the MCU block for this level of chrominance subsampling.
    ///
    /// This is equal to `8 * self.width()`.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(turbojpeg::Subsamp::Sub2x1.mcu_width(), 16);
    /// ```
    #[doc(alias = "tjMCUWidth")]
    pub fn mcu_width(self) -> usize {
        8 * self.width()
    }

    /// Get the height of the MCU block for this level of chrominance subsampling.
    ///
    /// This is equal to `8 * self.height()`.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(turbojpeg::Subsamp::Sub2x1.mcu_height(), 8);
    /// ```
    #[doc(alias = "tjMCUHeight")]
    pub fn mcu_height(self) -> usize {
        8 * self.height()
    }

    /// Get the size of the MCU block for this level of chrominance subsampling as (width, height).
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(turbojpeg::Subsamp::Sub2x1.mcu_size(), (16, 8));
    /// ```
    #[doc(alias = "tjMCUWidth")]
    #[doc(alias = "tjMCUHeight")]
    pub fn mcu_size(self) -> (usize, usize) {
        let (width, height) = self.size();
        (8 * width, 8 * height)
    }

    /// Get the horizontal subsampling factor.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(turbojpeg::Subsamp::Sub2x1.width(), 2);
    /// ```
    pub fn width(self) -> usize {
        self.size().0
    }

    /// Get the vertical subsampling factor.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(turbojpeg::Subsamp::Sub2x1.height(), 1);
    /// ```
    pub fn height(self) -> usize {
        self.size().1
    }

    /// Get the horizontal and vertical subsampling factors.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(turbojpeg::Subsamp::Sub2x1.size(), (2, 1));
    /// ```
    pub fn size(self) -> (usize, usize) {
        match self {
            Self::None => (1, 1),
            Self::Sub2x1 => (2, 1),
            Self::Sub2x2 => (2, 2),
            Self::Gray => (1, 1),
            Self::Sub1x2 => (1, 2),
            Self::Sub4x1 => (4, 1),
            Self::Sub1x4 => (1, 4),
            // NOTE: `tj3JPEGBufSize()` treats unknown subsampling as `TJSAMP_444` (aka 1x1
            // subsampling), because "a buffer large enough to hold a JPEG image with no
            // subsampling should also be large enough to hold a JPEG image with an arbitrary level
            // of subsampling" (from doc comment to `tj3JPEGBufSize()` in turbojpeg.h).
            Self::Unknown => (1, 1),
        }
    }
}


/// JPEG colorspaces.
#[doc(alias = "TJCS")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Colorspace {
    /// RGB colorspace.
    ///
    /// When compressing the JPEG image, the R, G, and B components in the source image are
    /// reordered into image planes, but no colorspace conversion or subsampling is performed. RGB
    /// JPEG images can be decompressed to any of the extended RGB pixel formats or grayscale, but
    /// they cannot be decompressed to YUV images.
    #[doc(alias = "TJCS_RGB")]
    RGB = raw::TJCS_TJCS_RGB,

    /// YCbCr colorspace.
    ///
    /// YCbCr is not an absolute colorspace but rather a mathematical transformation of RGB
    /// designed solely for storage and transmission. YCbCr images must be converted to RGB before
    /// they can actually be displayed. In the YCbCr colorspace, the Y (luminance) component
    /// represents the black & white portion of the original image, and the Cb and Cr (chrominance)
    /// components represent the color portion of the original image. Originally, the analog
    /// equivalent of this transformation allowed the same signal to drive both black & white and
    /// color televisions, but JPEG images use YCbCr primarily because it allows the color data to
    /// be optionally subsampled for the purposes of reducing bandwidth or disk space. YCbCr is the
    /// most common JPEG colorspace, and YCbCr JPEG images can be compressed from and decompressed
    /// to any of the extended RGB pixel formats or grayscale, or they can be decompressed to YUV
    /// planar images.
    #[doc(alias = "TJCS_YCbCr")]
    YCbCr = raw::TJCS_TJCS_YCbCr,

    /// Grayscale colorspace.
    ///
    /// The JPEG image retains only the luminance data (Y component), and any color data from the
    /// source image is discarded. Grayscale JPEG images can be compressed from and decompressed to
    /// any of the extended RGB pixel formats or grayscale, or they can be decompressed to YUV
    /// planar images.
    #[doc(alias = "TJCS_GRAY")]
    Gray = raw::TJCS_TJCS_GRAY,

    /// CMYK colorspace.
    ///
    /// When compressing the JPEG image, the C, M, Y, and K components in the source image are
    /// reordered into image planes, but no colorspace conversion or subsampling is performed. CMYK
    /// JPEG images can only be decompressed to CMYK pixels.
    #[doc(alias = "TJCS_CMYK")]
    CMYK = raw::TJCS_TJCS_CMYK,

    /// YCCK colorspace.
    ///
    /// YCCK (AKA "YCbCrK") is not an absolute colorspace but rather a mathematical transformation
    /// of CMYK designed solely for storage and transmission. It is to CMYK as YCbCr is to RGB.
    /// CMYK pixels can be reversibly transformed into YCCK, and as with YCbCr, the chrominance
    /// components in the YCCK pixels can be subsampled without incurring major perceptual loss.
    /// YCCK JPEG images can only be compressed from and decompressed to CMYK pixels.
    #[doc(alias = "TJCS_YCCK")]
    YCCK = raw::TJCS_TJCS_YCCK,
}

impl Colorspace {
    pub(crate) fn from_int(colorspace: libc::c_int) -> Result<Colorspace> {
        Ok(match colorspace as libc::c_uint {
            raw::TJCS_TJCS_RGB => Colorspace::RGB,
            raw::TJCS_TJCS_YCbCr => Colorspace::YCbCr,
            raw::TJCS_TJCS_GRAY => Colorspace::Gray,
            raw::TJCS_TJCS_CMYK => Colorspace::CMYK,
            raw::TJCS_TJCS_YCCK => Colorspace::YCCK,
            other => return Err(Error::BadColorspace(other)),
        })
    }
}


/// Specialized `Result` type for TurboJPEG.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur in TurboJPEG.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// TurboJPEG returned an error message.
    #[error("TurboJPEG error: {0}")]
    TurboJpegError(String),
    
    /// TurboJPEG unexpectedly returned a null pointer, prehaps because it ran out of memory.
    #[error("TurboJPEG returned null pointer")]
    Null,

    /// TurboJPEG returned a chrominance subsampling variant that is not known by this crate.
    #[error("TurboJPEG returned unknown subsampling option: {0}")]
    BadSubsamp(i32),

    /// TurboJPEG returned a colorspace variant that is not known by this crate.
    #[error("TurboJPEG returned unknown colorspace: {0}")]
    BadColorspace(u32),

    /// The given integer value overflowed when converted into type expected by TurboJPEG.
    #[error("integer value {0:?} overflowed")]
    IntegerOverflow(&'static str),

    /// When decompressing, the output image is too small for the input JPEG image.
    #[error("output image is too small for image of size {0}x{1}")]
    OutputTooSmall(i32, i32),
}

