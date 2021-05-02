//! Rust bindings for TurboJPEG, which provides simple and fast compression/decompression of JPEG
//! images.
//!
//! Compression is provided by [`Compressor`] and decompression by [`Decompressor`].

pub extern crate turbojpeg_sys as sys;
pub extern crate libc;

pub use common::{PixelFormat, Subsamp, Colorspace, Result, Error};
pub use compress::Compressor;
pub use decompress::{Decompressor, DecompressHeader};

mod common;
mod compress;
mod decompress;

/// An image with pixels of type `T` (typically `&[u8]` or `&mut [u8]`).
///
/// Data for pixel in row `x` and column `y` is stored in `pixels` at offset `y*pitch +
/// x*format.size()`.
#[derive(Debug, Copy, Clone)]
pub struct Image<T> {
    /// Pixel data of the image (typically `&[u8]` or `&mut [u8]`).
    pub pixels: T,
    /// Width of the image in pixels (number of columns).
    pub width: usize,
    /// Pitch (stride) defines the size of one image row in bytes. Overlapping rows are not
    /// supported, so we require that `pitch >= width * format.size()`.
    pub pitch: usize,
    /// Height of the image in pixels (number of rows).
    pub height: usize,
    /// Format of pixels in memory, determines the color format (RGB, RGBA, grayscale or CMYK) and
    /// the memory layout (RGB, BGR, RGBA, ...).
    pub format: PixelFormat,
}

impl<T> Image<T> {
    pub(crate) fn assert_valid(&self, pixels_len: usize) {
        let Image { pixels: _, width, pitch, height, format } = *self;
        assert!(pitch >= width*format.size(),
            "pitch {} is too small for width {} and pixel format {:?}", pitch, width, format);
        assert!(height == 0 || pitch*(height - 1) + width*format.size() <= pixels_len,
            "pixels length {} is too small for width {}, height {}, pitch {} and pixel format {:?}",
            pixels_len, width, height, pitch, format);
    }
}
