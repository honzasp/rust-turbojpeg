use std::ops::{Deref, DerefMut};
use crate::common::PixelFormat;

/// An image with pixels of type `T`.
///
/// Three variants of this type are commonly used:
///
/// - `Image<&[u8]>`: immutable reference to image data (input image for compression by
/// [`Compressor`][crate::Compressor])
/// - `Image<&mut [u8]>`: mutable reference to image data (output image for decompression by
/// [`Decompressor`][crate::Compressor]).
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

/// A yuv image with pixels of type `T`.
///
/// Three variants of this type are commonly used:
///
/// - `YUVImage<&[u8]>`: immutable reference to yuv image data (input image for compression by
/// [`Compressor`][crate::Compressor])
/// - `YUVImage<&mut [u8]>`: mutable reference to yuv image data (output image for decompression by
/// [`Decompressor`][crate::Compressor]).
/// - `YUVImage<Vec<u8>>`: owned yuv image data (you can convert it to a reference using
/// [`.as_deref()`][Image::as_deref] or [`.as_deref_mut()`][Image::as_deref_mut]).
pub struct YUVImage<T> {
    /// Pixel data of the image (typically `&[u8]`, `&mut [u8]` or `Vec<u8>`).
    pub pixels: T,
    /// Width of the image in pixels (number of columns).
    pub width: usize,
    /// Pad the width of each line in each plane of the YUV image will be
    /// padded to the nearest multiple of this number of bytes (must be a power of
    /// 2.)  To generate images suitable for X Video, <tt>pad</tt> should be set to 4
    pub pad: usize,
    /// Height of the image in pixels (number of rows).
    pub height: usize,
}

impl<T> YUVImage<T> {
    /// Converts from `&YUVImage<T>` to `YUVImage<&T::Target>`.
    ///
    /// In particular, you can use this to get `YUVImage<&[u8]>` from `YUVImage<Vec<u8>>`.
    pub fn as_deref(&self) -> YUVImage<&T::Target> where T: Deref {
        YUVImage {
            pixels: self.pixels.deref(),
            width: self.width,
            pad: self.pad,
            height: self.height,
        }
    }

    /// Converts from `&mut YUVImage<T>` to `YUVImage<&mut T::Target>`.
    ///
    /// In particular, you can use this to get `YUVImage<&mut [u8]>` from `YUVImage<Vec<u8>>`.
    pub fn as_deref_mut(&mut self) -> YUVImage<&mut T::Target> where T: DerefMut {
        YUVImage {
            pixels: self.pixels.deref_mut(),
            width: self.width,
            pad: self.pad,
            height: self.height,
        }
    }
}