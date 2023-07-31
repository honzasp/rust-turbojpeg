use std::ops::{Deref, DerefMut};
use crate::{common::PixelFormat, Subsamp, yuv_pixels_len};

/// An image with pixels of type `T`.
///
/// Three variants of this type are commonly used:
///
/// - `Image<&[u8]>`: immutable reference to image data (input image for compression by
/// [`Compressor`][crate::Compressor])
/// - `Image<&mut [u8]>`: mutable reference to image data (output image for decompression by
/// [`Decompressor`][crate::Decompressor]).
/// - `Image<Vec<u8>>`: owned image data (you can convert it to a reference using
/// [`.as_deref()`][Image::as_deref] or [`.as_deref_mut()`][Image::as_deref_mut]).
///
/// Data for pixel in column `x` and row `y` is stored in `pixels` at offset `y*pitch +
/// x*format.size()`.
#[derive(Debug, Copy, Clone)]
pub struct Image<T> {
    /// Pixel data of the image (typically `&[u8]`, `&mut [u8]` or `Vec<u8>`).
    pub pixels: T,
    /// Width of the image in pixels (number of columns).
    pub width: usize,
    /// Pitch (stride) defines the size of one image row in bytes. Overlapping rows are not
    /// supported, we require that `pitch >= width * format.size()`.
    pub pitch: usize,
    /// Height of the image in pixels (number of rows).
    pub height: usize,
    /// Format of pixels in memory, determines the color format (RGB, RGBA, grayscale or CMYK) and
    /// the memory layout (RGB, BGR, RGBA, ...).
    pub format: PixelFormat,
}

impl<T> Image<T> {
    /// Converts from `&Image<T>` to `Image<&T::Target>`.
    ///
    /// In particular, you can use this to get `Image<&[u8]>` from `Image<Vec<u8>>`.
    pub fn as_deref(&self) -> Image<&T::Target> where T: Deref {
        Image {
            pixels: self.pixels.deref(),
            width: self.width,
            pitch: self.pitch,
            height: self.height,
            format: self.format,
        }
    }

    /// Converts from `&mut Image<T>` to `Image<&mut T::Target>`.
    ///
    /// In particular, you can use this to get `Image<&mut [u8]>` from `Image<Vec<u8>>`.
    pub fn as_deref_mut(&mut self) -> Image<&mut T::Target> where T: DerefMut {
        Image {
            pixels: self.pixels.deref_mut(),
            width: self.width,
            pitch: self.pitch,
            height: self.height,
            format: self.format,
        }
    }

    pub(crate) fn assert_valid(&self, pixels_len: usize) {
        let Image { pixels: _, width, pitch, height, format } = *self;
        assert!(pitch >= width*format.size(),
            "pitch {} is too small for width {} and pixel format {:?}", pitch, width, format);
        assert!(height == 0 || pitch*(height - 1) + width*format.size() <= pixels_len,
            "pixels length {} is too small for width {}, height {}, pitch {} and pixel format {:?}",
            pixels_len, width, height, pitch, format);
    }
}

impl Image<Vec<u8>> {
    /// Generates an image of the Mandelbrot set.
    ///
    /// The generated image has the given width and height and uses the given pixel format. This
    /// method is intended for testing and demonstration purposes.
    ///
    /// # Example
    ///
    /// ```
    /// let image = turbojpeg::Image::mandelbrot(200, 200, turbojpeg::PixelFormat::BGRA);
    /// assert_eq!((image.width, image.height), (200, 200));
    /// assert_eq!(image.format, turbojpeg::PixelFormat::BGRA);
    /// ```
    pub fn mandelbrot(width: usize, height: usize, format: PixelFormat) -> Image<Vec<u8>> {
        // determine mapping from pixels to the complex plane

        let radius = 2.;
        let scale = usize::max(width, height) as f64 / (2. * radius);
        let origin_x = width as f64 * 0.5;
        let origin_y = height as f64 * 0.5;

        let pixel_to_set = |pixel_x: usize, pixel_y: usize| -> (f64, f64) {
            let (pixel_x, pixel_y) = (pixel_x as f64 + 0.5, pixel_y as f64 + 0.5);
            let set_x = (pixel_x - origin_x) / scale;
            let set_y = (pixel_y - origin_y) / scale;
            (set_x, set_y)
        };

        // evaluate the mandelbrot set function

        fn eval_set(set_x: f64, set_y: f64) -> f64 {
            let max_iters = 100;
            let (mut x, mut y) = (set_x, set_y);
            let mut iters = 0;
            while x*x + y*y <= 4. && iters < max_iters {
                let next_x = x*x - y*y + set_x;
                let next_y = 2.*x*y + set_y;
                x = next_x;
                y = next_y;
                iters += 1;
            }
            1. - 0.99f64.powi(iters)
        }

        // convert the f64 values to pixel values

        fn assign_rgba(r: usize, g: usize, b: usize, a: Option<usize>, data: &mut [u8], value: f64) {
            data[b] = quantize(f64::clamp(f64::min(3.*value, 3. - 3.*value), 0., 1.));
            data[r] = quantize(f64::clamp(f64::max(1. - 3.*value, 3.*value - 2.), 0., 1.));
            data[g] = quantize(f64::clamp(value, 0., 1.));
            if let Some(a) = a { data[a] = 255; }
        }

        fn assign_gray(data: &mut [u8], value: f64) {
            data[0] = quantize(value);
        }

        fn quantize(value: f64) -> u8 {
            (value * 255.) as u8
        }


        let pixel_size = format.size();
        let assign_fn: &dyn Fn(&mut [u8], f64) = match format {
            PixelFormat::RGB =>
                &|data, value| assign_rgba(0,1,2,None, data, value),
            PixelFormat::BGR =>
                &|data, value| assign_rgba(2,1,0,None, data, value),
            PixelFormat::RGBX | PixelFormat::RGBA =>
                &|data, value| assign_rgba(0,1,2,Some(3), data, value),
            PixelFormat::BGRX | PixelFormat::BGRA =>
                &|data, value| assign_rgba(2,1,0,Some(3), data, value),
            PixelFormat::XRGB | PixelFormat::ARGB =>
                &|data, value| assign_rgba(1,2,3,Some(0), data, value),
            PixelFormat::XBGR | PixelFormat::ABGR =>
                &|data, value| assign_rgba(3,2,1,Some(0), data, value),
            PixelFormat::GRAY =>
                &assign_gray,
            PixelFormat::CMYK =>
                &|data, value| assign_rgba(0,1,2,Some(3), data, value),
        };

        // generate the image

        let align = 32;
        let pitch = (pixel_size * width + align - 1) / align * align;
        let mut pixels = vec![0; pitch * height];

        for y in 0..height {
            for x in 0..width {
                let (set_x, set_y) = pixel_to_set(x, y);
                let value = eval_set(set_x, set_y);
                assign_fn(&mut pixels[y*pitch + pixel_size*x..], value);
            }
        }

        Image { pixels, width, pitch, height, format }
    }
}

/// A YUV (YCbCr) planar image with pixels of type `T`.
///
/// This type stores an image in the JPEG color transform YCbCr (also called "YUV"). The image data
/// first stores the Y plane, then the U (Cb) plane, and then the V (Cr) plane.
///
/// Two variants of this type are commonly used:
///
/// - `YuvImage<&mut [u8]>`: mutable reference to YUV image data (output image for decompression by
/// [`Decompressor`][crate::Decompressor]).
/// - `YuvImage<Vec<u8>>`: owned yuv image data (you can convert it to a reference using
/// [`.as_deref()`][YuvImage::as_deref] or [`.as_deref_mut()`][YuvImage::as_deref_mut]).
///
/// # Image format
///
/// The size of each image plane is determined by the [width][Self::width], [height][Self::height],
/// [chrominance subsampling][Self::subsamp] and [row alignment][Self::align] of the image:
///
/// - [Luminance (Y) plane width][Self::y_width()] is the image width padded to the nearest
/// multiple of the [horizontal subsampling factor][Subsamp::width()].
/// - [Luminance (Y) plane height][Self::y_height()] is the image height padded to the nearest
/// multiple of the [vertical subsampling factor][Subsamp::height()].
/// - [Chrominance (U and V) plane width][Self::uv_width()] is the luminance plane width divided by
/// the horizontal subsampling factor.
/// - [Chrominance (U and V) plane height][Self::uv_height()] is the luminance plane height divided
/// by the vertical subsampling factor.
/// - Each row is further padded to the nearest multiple of the [row alignment][Self::align].
///
/// ## Example
///
/// For example, if the source image is 35 x 35 pixels and [`Sub2x1`][Subsamp::Sub2x1] subsampling
/// is used, then the luminance plane would be 36 x 35 bytes, and each of the chrominance planes
/// would be 18 x 35 bytes. If you specify a row alignment of 4 bytes on top of this, then the
/// luminance plane would be 36 x 35 bytes, and each of the chrominance planes would be 20 x 35
/// bytes.
///
/// ```rust
/// let img1 = turbojpeg::YuvImage {
///     pixels: (),
///     width: 35,
///     align: 1,
///     height: 35,
///     subsamp: turbojpeg::Subsamp::Sub2x1,
/// };
/// assert_eq!(img1.y_size(), (36, 35));
/// assert_eq!(img1.uv_size(), (18, 35));
///
/// let img2 = turbojpeg::YuvImage { align: 4, ..img1 };
/// assert_eq!(img2.y_size(), (36, 35));
/// assert_eq!(img2.uv_size(), (20, 35));
/// ```
pub struct YuvImage<T> {
    /// Pixel data of the image (typically `&mut [u8]` or `Vec<u8>`).
    pub pixels: T,
    /// Width of the image in pixels (number of columns).
    pub width: usize,
    /// Row alignment (in bytes) of the YUV image (must be a power of 2.) Each row in each plane of
    /// the YUV image will be padded to the nearest multiple of `align`.
    pub align: usize,
    /// Height of the image in pixels (number of rows).
    pub height: usize,
    /// The level of chrominance subsampling used in the YUV image.
    pub subsamp: Subsamp,
}

impl<T> YuvImage<T> {
    /// Converts from `&YuvImage<T>` to `YuvImage<&T::Target>`.
    ///
    /// In particular, you can use this to get `YuvImage<&[u8]>` from `YuvImage<Vec<u8>>`.
    pub fn as_deref(&self) -> YuvImage<&T::Target> where T: Deref {
        YuvImage {
            pixels: self.pixels.deref(),
            width: self.width,
            align: self.align,
            height: self.height,
            subsamp: self.subsamp,
        }
    }

    /// Converts from `&mut YuvImage<T>` to `YuvImage<&mut T::Target>`.
    ///
    /// In particular, you can use this to get `YuvImage<&mut [u8]>` from `YuvImage<Vec<u8>>`.
    pub fn as_deref_mut(&mut self) -> YuvImage<&mut T::Target> where T: DerefMut {
        YuvImage {
            pixels: self.pixels.deref_mut(),
            width: self.width,
            align: self.align,
            height: self.height,
            subsamp: self.subsamp,
        }
    }

    /// Computes width of the luminance (Y) plane.
    ///
    /// This is the [image width][Self::width] padded to the nearest multiple of the [horizontal subsampling
    /// factor][Subsamp::width()] and then aligned to the [row alignment][Self::align].
    pub fn y_width(&self) -> usize {
        let width = next_multiple_of(self.width, self.subsamp.width());
        next_multiple_of(width, self.align)
    }

    /// Computes height of the luminance (Y) plane.
    ///
    /// This is the [image height][Self::height] padded to the nearest multiple of the [vertical
    /// subsampling factor][Subsamp::height()].
    pub fn y_height(&self) -> usize {
        next_multiple_of(self.height, self.subsamp.height())
    }

    /// Computes size of the luminance (Y) plane.
    pub fn y_size(&self) -> (usize, usize) {
        (self.y_width(), self.y_height())
    }

    /// Computes width of each chrominance (U, V) plane.
    ///
    /// This is the [Y plane width][Self::y_width()] divided by the [horizontal subsampling
    /// factor][Subsamp::width()] and then aligned to the [row alignment][Self::align].
    pub fn uv_width(&self) -> usize {
        let width = div_ceil(self.width, self.subsamp.width());
        next_multiple_of(width, self.align)
    }

    /// Computes height of each chrominance (U, V) plane.
    ///
    /// This is the [Y plane height][Self::y_height()] divided by the [vertical subsampling
    /// factor][Subsamp::height()].
    pub fn uv_height(&self) -> usize {
        div_ceil(self.height, self.subsamp.height())
    }

    /// Computes size of each chrominance (U, V) plane.
    pub fn uv_size(&self) -> (usize, usize) {
        (self.uv_width(), self.uv_height())
    }

    pub(crate) fn assert_valid(&self, pixels_len: usize) {
        let YuvImage { pixels: _, width, align, height, subsamp } = *self;
        let min_yuv_pixels_len = yuv_pixels_len(width, align, height, subsamp).unwrap();
        assert!(min_yuv_pixels_len <= pixels_len,
            "YUV pixels length {} is too small for width {}, height {}, align {} and subsamp {:?}",
            pixels_len, width, height, align, subsamp);
    }
}

// TODO: these two functions will eventually be stabilized into the standard library

fn next_multiple_of(n: usize, divisor: usize) -> usize {
    div_ceil(n, divisor) * divisor
}

fn div_ceil(n: usize, divisor: usize) -> usize {
    (n + divisor - 1) / divisor
}
