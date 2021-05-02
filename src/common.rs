use std::ffi::CStr;

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
    RGB = sys::TJPF_TJPF_RGB,

    /// BGR pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 3-byte pixels in the order
    /// B, G, R from lowest to highest byte address within each pixel.
    #[doc(alias = "TJPF_BGR")]
    BGR = sys::TJPF_TJPF_BGR,

    /// RGBX pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// R, G, B from lowest to highest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_RGBX")]
    RGBX = sys::TJPF_TJPF_RGBX,

    /// BGRX pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// B, G, R from lowest to highest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_BGRX")]
    BGRX = sys::TJPF_TJPF_BGRX,

    /// XBGR pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// R, G, B from highest to lowest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_XBGR")]
    XBGR = sys::TJPF_TJPF_XBGR,

    /// XRGB pixel format.
    ///
    /// The red, green, and blue components in the image are stored in 4-byte pixels in the order
    /// B, G, R from highest to lowest byte address within each pixel. The X component is ignored
    /// when compressing and undefined when decompressing.
    #[doc(alias = "TJPF_XRGB")]
    XRGB = sys::TJPF_TJPF_XRGB,

    /// Grayscale pixel format.
    ///
    /// Each 1-byte pixel represents a luminance (brightness) level from 0 to 255.
    #[doc(alias = "TJPF_GRAY")]
    GRAY = sys::TJPF_TJPF_GRAY,

    /// RGBA pixel format.
    ///
    /// This is the same as [`PixelFormat::RGBX`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_RGBA")]
    RGBA = sys::TJPF_TJPF_RGBA,

    /// BGRA pixel format.
    ///
    /// This is the same as [`PixelFormat::BGRX`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_BGRA")]
    BGRA = sys::TJPF_TJPF_BGRA,

    /// ABGR pixel format.
    ///
    /// This is the same as [`PixelFormat::XBGR`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_ABGR")]
    ABGR = sys::TJPF_TJPF_ABGR,

    /// ARGB pixel format.
    ///
    /// This is the same as [`PixelFormat::ARGB`], except that when decompressing, the X component
    /// is guaranteed to be 0xFF, which can be interpreted as an opaque alpha channel.
    #[doc(alias = "TJPF_ARGB")]
    ARGB = sys::TJPF_TJPF_ARGB,

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
    CMYK = sys::TJPF_TJPF_CMYK,
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

    pub fn from_i32(format: i32) -> Result<PixelFormat> {
        Ok(match format {
            sys::TJPF_TJPF_RGB => PixelFormat::RGB,
            sys::TJPF_TJPF_BGR => PixelFormat::BGR,
            sys::TJPF_TJPF_RGBX => PixelFormat::RGBX,
            sys::TJPF_TJPF_BGRX => PixelFormat::BGRX,
            sys::TJPF_TJPF_XBGR => PixelFormat::XBGR,
            sys::TJPF_TJPF_XRGB => PixelFormat::XRGB,
            sys::TJPF_TJPF_GRAY => PixelFormat::GRAY,
            sys::TJPF_TJPF_RGBA => PixelFormat::RGBA,
            sys::TJPF_TJPF_BGRA => PixelFormat::BGRA,
            sys::TJPF_TJPF_ABGR => PixelFormat::ABGR,
            sys::TJPF_TJPF_ARGB => PixelFormat::ARGB,
            sys::TJPF_TJPF_CMYK => PixelFormat::CMYK,
            other => return Err(Error::BadPixelFormat(other)),
        })
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
#[repr(u32)]
pub enum Subsamp {
    /// No chrominance subsampling (4:4:4);
    ///
    /// The JPEG or YUV image will contain one chrominance component for every pixel in the source
    /// image.
    #[doc(alias = "TJSAMP_444")]
    None = sys::TJSAMP_TJSAMP_444,

    /// 2x1 chrominance subsampling (4:2:2).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 2x1 block of pixels
    /// in the source image.
    #[doc(alias = "TJSAMP_422")]
    Sub2x1 = sys::TJSAMP_TJSAMP_422,

    /// 2x2 chrominance subsampling (4:2:0).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 2x2 block of pixels
    /// in the source image.
    #[doc(alias = "TJSAMP_420")]
    Sub2x2 = sys::TJSAMP_TJSAMP_420,

    /// Grayscale.
    ///
    /// The JPEG or YUV image will contain no chrominance components.
    #[doc(alias = "TJSAMP_GRAY")]
    Gray = sys::TJSAMP_TJSAMP_GRAY,

    /// 1x2 chrominance subsampling (4:4:0).
    ///
    /// The JPEG or YUV image will contain one chrominance component for every 1x2 block of pixels
    /// in the source image.
    ///
    /// # Note
    ///
    /// 4:4:0 subsampling is not fully accelerated in libjpeg-turbo.
    #[doc(alias = "TJSAMP_440")]
    Sub1x2 = sys::TJSAMP_TJSAMP_440,

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
    Sub4x1 = sys::TJSAMP_TJSAMP_411,
}

impl Subsamp {
    pub fn from_u32(subsamp: u32) -> Result<Subsamp> {
        Ok(match subsamp {
            sys::TJSAMP_TJSAMP_444 => Subsamp::None,
            sys::TJSAMP_TJSAMP_422 => Subsamp::Sub2x1,
            sys::TJSAMP_TJSAMP_420 => Subsamp::Sub2x2,
            sys::TJSAMP_TJSAMP_GRAY => Subsamp::Gray,
            sys::TJSAMP_TJSAMP_440 => Subsamp::Sub1x2,
            sys::TJSAMP_TJSAMP_411 => Subsamp::Sub4x1,
            other => return Err(Error::BadSubsamp(other)),
        })
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
    RGB = sys::TJCS_TJCS_RGB,

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
    YCbCr = sys::TJCS_TJCS_YCbCr,

    /// Grayscale colorspace.
    ///
    /// The JPEG image retains only the luminance data (Y component), and any color data from the
    /// source image is discarded. Grayscale JPEG images can be compressed from and decompressed to
    /// any of the extended RGB pixel formats or grayscale, or they can be decompressed to YUV
    /// planar images.
    #[doc(alias = "TJCS_GRAY")]
    Gray = sys::TJCS_TJCS_GRAY,

    /// CMYK colorspace.
    ///
    /// When compressing the JPEG image, the C, M, Y, and K components in the source image are
    /// reordered into image planes, but no colorspace conversion or subsampling is performed. CMYK
    /// JPEG images can only be decompressed to CMYK pixels.
    #[doc(alias = "TJCS_CMYK")]
    CMYK = sys::TJCS_TJCS_CMYK,

    /// YCCK colorspace.
    ///
    /// YCCK (AKA "YCbCrK") is not an absolute colorspace but rather a mathematical transformation
    /// of CMYK designed solely for storage and transmission. It is to CMYK as YCbCr is to RGB.
    /// CMYK pixels can be reversibly transformed into YCCK, and as with YCbCr, the chrominance
    /// components in the YCCK pixels can be subsampled without incurring major perceptual loss.
    /// YCCK JPEG images can only be compressed from and decompressed to CMYK pixels.
    #[doc(alias = "TJCS_YCCK")]
    YCCK = sys::TJCS_TJCS_YCCK,
}

impl Colorspace {
    pub fn from_u32(colorspace: u32) -> Result<Colorspace> {
        Ok(match colorspace {
            sys::TJCS_TJCS_RGB => Colorspace::RGB,
            sys::TJCS_TJCS_YCbCr => Colorspace::YCbCr,
            sys::TJCS_TJCS_GRAY => Colorspace::Gray,
            sys::TJCS_TJCS_CMYK => Colorspace::CMYK,
            sys::TJCS_TJCS_YCCK => Colorspace::YCCK,
            other => return Err(Error::BadColorspace(other)),
        })
    }
}


/// Specialized `Result` type for TurboJPEG.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur in TurboJPEG.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("TurboJPEG error: {0}")]
    TurboJpegError(String),
    #[error("TurboJPEG returned null pointer")]
    Null(),
    #[error("TurboJPEG returned unknown pixel format: {0}")]
    BadPixelFormat(i32),
    #[error("TurboJPEG returned unknown subsampling option: {0}")]
    BadSubsamp(u32),
    #[error("TurboJPEG returned unknown colorspace: {0}")]
    BadColorspace(u32),
    #[error("integer value {0:?} overflowed")]
    IntegerOverflow(&'static str),
}

pub(crate) unsafe fn get_error(handle: sys::tjhandle) -> Error {
    let msg = CStr::from_ptr(sys::tjGetErrorStr2(handle));
    Error::TurboJpegError(msg.to_string_lossy().into_owned())
}
