//! Rust bindings for TurboJPEG, which provides simple and fast compression/decompression of JPEG
//! images.
//!
//! # High-level usage with image-rs
//! 
//! To easily encode and decode images from the [`image`][image-rs] crate, please
//! enable the optional dependency by adding this to the `[dependencies]` section of
//! your `Cargo.toml`:
//! 
//! ```toml
//! turbojpeg = {version = "^0.2", features = ["image"]}
//! ```
//! 
//! Then you can use the functions [`decompress_image`] and
//! [`compress_image`] to easily decode and encode JPEG:
//! 
//! ```rust
//! // create an `image::RgbImage`
//! let image: image::RgbImage = ...;
//! // compress `image` into JPEG with quality 95 and no chrominance subsampling
//! let jpeg_data = turbojpeg::compress_image(&image, 95, turbojpeg::Subsamp::None)?;
//! 
//! // decompress `jpeg_data` into an `image::RgbImage`
//! let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data);
//! ```
//! 
//! This crate supports these image types:
//! 
//! - [`RgbImage`][::image::RgbImage]
//! - [`RgbaImage`][::image::RgbaImage] (JPEG does not support alpha channel, so alpha is ignored
//!   when encoding and set to 255 when decoding)
//! - [`GrayImage`][::image::GrayImage]
//! 
//! [image-rs]: https://docs.rs/image/*/image/index.html
//! 
//! # Low-level usage with `Compressor`/`Decompressor`
//! 
//! Use [`Compressor`] to compress raw pixel data into JPEG
//! (see [`examples/compressor.rs`][compressor-example] for full example):
//!
//! [compressor-example]: https://github.com/honzasp/rust-turbojpeg/blob/master/examples/compressor.rs
//! 
//! ```rust
//! use turbojpeg::{Compressor, Image, PixelFormat};
//! 
//! // prepare the raw pixel data
//! let width: usize = ...;
//! let height: usize = ...;
//! let pixels: Vec<u8> = ...;
//! 
//! // initialize a Compressor
//! let mut compressor = Compressor::new()?;
//! 
//! // create an Image that bundles a reference to the raw pixel data (as &[u8])
//! // with information about the image format
//! let image = Image {
//!     // &[u8] reference to the pixel data
//!     pixels: pixels.as_slice(),
//!     // width of the image in pixels
//!     width: width,
//!     // size of the image row in bytes (also called "stride")
//!     pitch: 3 * width,
//!     // height of the image in pixels
//!     height: height,
//!     // format of the pixel data
//!     format: PixelFormat::RGB,
//! };
//!
//! // compress the Image to a Vec<u8> of JPEG data
//! let jpeg_data = compressor.compress_to_vec(image)?;
//! ```
//! 
//! To decompress JPEG data into a raw pixel data, use [`Decompressor`] (full example in
//! [`examples/decompressor.rs`][decompressor-example]):
//!
//! [decompressor-example]: https://github.com/honzasp/rust-turbojpeg/blob/master/examples/decompressor.rs
//! 
//! ```rust
//! use turbojpeg::{Decompressor, Image, PixelFormat};
//! 
//! // get the JPEG data
//! let jpeg_data: &[u8] = ...;
//! 
//! // initialize a Decompressor
//! let mut decompressor = Decompressor::new()?;
//! 
//! // read the JPEG header with image size
//! let header = decompressor.read_header(jpeg_data)?;
//! let (width, height) = (header.width, header.height);
//! 
//! // prepare a storage for the raw pixel data
//! let mut pixels = vec![0; 3*width*height];
//! let image = Image {
//!     // &mut [u8] reference to the image data
//!     pixels: pixels.as_mut_slice(),
//!     width: width,
//!     pitch: 3 * width,
//!     height: height,
//!     format: PixelFormat::RGB,
//! };
//! 
//! // decompress the JPEG data 
//! decompressor.decompress_to_slice(jpeg_data, image)?;
//! 
//! // use the raw pixel data
//! println!("{:?}", &pixels[0..9]);
//! ```
//! 
//! # Features
//!
//! - `image`: enables the optional dependency on the [`image`][image-rs] crate.
//! - `pkg-config`: uses pkg-config to find the `libturbojpeg` library.
//! - `bindgen`: uses [bindgen] to generate the `libturbojpeg` bindings.
//!
//! [bindgen]: https://rust-lang.github.io/rust-bindgen/
#![warn(missing_docs)]

pub extern crate turbojpeg_sys as sys;
pub extern crate libc;

mod common;
mod compress;
mod decompress;
pub use common::{PixelFormat, Subsamp, Colorspace, Result, Error};
pub use compress::Compressor;
pub use decompress::{Decompressor, DecompressHeader};

use std::ops::{Deref, DerefMut};

#[cfg(feature = "image")]
mod image;
#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub use self::image::{JpegPixel, compress_image, decompress_image};

/// An image with pixels of type `T`.
///
/// Three variants of this type are commonly used:
///
/// - `Image<&[u8]>`: immutable reference to image data (source image for compression by
/// [`Compressor`])
/// - `Image<&mut [u8]>`: mutable reference to image data (destination image for decompression by
/// [`Decompressor`]).
/// - `Image<Vec<u8>>`: owned image data (can be converted to a reference using
/// [`.as_deref()`][Image::as_deref] or [`.as_deref_mut`][Image::as_deref_mut].
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
    /// Converts from `&Image<T>` to `Image<&T::Target>`.
    ///
    /// In particular, this can be used to get `Image<&[u8]>` from `Image<Vec<u8>>`.
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
    /// In particular, this can be used to get `Image<&mut [u8]>` from `Image<Vec<u8>>`.
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
