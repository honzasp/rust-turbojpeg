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

