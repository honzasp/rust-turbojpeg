use crate::Image;
use crate::buf::OwnedBuf;
use crate::compress::Compressor;
use crate::common::{PixelFormat, Result, Subsamp};
use crate::decompress::Decompressor;

/// Decompresses image from JPEG into an [`image::ImageBuffer`].
///
/// # Example
///
/// ```
/// // read JPEG data from file
/// let jpeg_data = std::fs::read("examples/parrots.jpg")?;
///
/// // decompress `jpeg_data` into an `image::RgbImage`
/// let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn decompress_image<P>(jpeg_data: &[u8]) -> Result<image::ImageBuffer<P, Vec<u8>>>
    where P: JpegPixel + 'static
{
    let mut decompressor = Decompressor::new()?;
    let header = decompressor.read_header(jpeg_data)?;

    let pitch = header.width * P::PIXEL_FORMAT.size();
    let mut image_data = vec![0; pitch * header.height];
    let image = Image {
        pixels: &mut image_data[..],
        width: header.width,
        pitch,
        height: header.height,
        format: P::PIXEL_FORMAT,
    };
    decompressor.decompress(jpeg_data, image)?;

    let image_buf = image::ImageBuffer::from_raw(
        header.width as u32,
        header.height as u32,
        image_data,
    ).unwrap();
    Ok(image_buf)
}

/// Compresses an [`image::ImageBuffer`] into JPEG.
///
/// `quality` controls the tradeoff between image quality and size of the compressed image. It
/// ranges from 1 (worst quality, smallest size) to 100 (best quality, largest size).
///
/// `subsamp` sets the level of chrominance subsampling of the compressed JPEG image (please see
/// the documentation of [`Subsamp`] for details). Use [`Subsamp::None`] for no subsampling
/// (highest quality).
///
/// # Example
///
/// ```
///
/// // create an `image::RgbImage`
/// let image = image::RgbImage::from_fn(256, 256, |x, y| image::Rgb([x as u8, y as u8, 128]));
///
/// // compress `image` into JPEG with quality 95 and 2x2 chrominance subsampling
/// let jpeg_data = turbojpeg::compress_image(&image, 95, turbojpeg::Subsamp::Sub2x2)?;
///
/// // write the JPEG to disk
/// std::fs::write(std::env::temp_dir().join("gradient.jpg"), &jpeg_data)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn compress_image<P>(
    image_buf: &image::ImageBuffer<P, Vec<u8>>,
    quality: i32,
    subsamp: Subsamp,
) -> Result<OwnedBuf>
    where P: JpegPixel + 'static
{
    let (width, height) = image_buf.dimensions();
    let format = P::PIXEL_FORMAT;
    let image = Image {
        pixels: &image_buf.as_raw()[..],
        width: width as usize,
        pitch: format.size() * width as usize,
        height: height as usize,
        format,
    };

    let mut compressor = Compressor::new()?;
    compressor.set_quality(quality);
    compressor.set_subsamp(subsamp);
    compressor.compress_to_owned(image)
}

/// Trait implemented for [`image::Pixel`s][image::Pixel] that correspond to a [`PixelFormat`] supported
/// by TurboJPEG.
pub trait JpegPixel: image::Pixel<Subpixel = u8> {
    /// The TurboJPEG pixel format that corresponds to this pixel type.
    const PIXEL_FORMAT: PixelFormat;
}

impl JpegPixel for image::Rgb<u8> {
    const PIXEL_FORMAT: PixelFormat = PixelFormat::RGB;
}
impl JpegPixel for image::Rgba<u8> {
    const PIXEL_FORMAT: PixelFormat = PixelFormat::RGBA;
}
impl JpegPixel for image::Luma<u8> {
    const PIXEL_FORMAT: PixelFormat = PixelFormat::GRAY;
}
