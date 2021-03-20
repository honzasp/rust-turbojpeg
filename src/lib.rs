pub extern crate turbojpeg_sys as sys;
pub extern crate libc;

pub use common::{PixelFormat, Subsamp, Colorspace, Result, Error};
pub use compress::Compressor;
pub use decompress::{Decompressor, DecompressHeader};

mod common;
mod compress;
mod decompress;

/// Slice of pixel data.
#[derive(Debug, Copy, Clone)]
pub struct Image<T> {
    pub pixels: T,
    pub width: usize,
    pub pitch: usize,
    pub height: usize,
    pub format: PixelFormat,
}

impl<T> Image<T> {
    pub(crate) fn assert_valid(&self, pixels_len: usize) {
        let Image { pixels: _, width, pitch, height, format } = *self;
        assert!(pitch >= width*format.size(),
            "pitch {} is too small for width {} and pixel format {:?}", pitch, width, format);
        assert!(height == 0 || pitch*(height - 1) + width*format.size() <= pixels_len);
    }
}
