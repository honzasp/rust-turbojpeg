//! Rust bindings for TurboJPEG, which provides simple and fast operations for JPEG images:
//!
//! - [Compression][compress()]: encode images into JPEG.
//! - [Decompression][decompress()]: decode JPEGs into pixels.
//! - [Lossless transformations][transform()]: apply basic geometric transformations (rotate, mirror,
//! ...) without going through decompression and compression, so that the image does not lose
//! quality.
//!
//! # Integration with image-rs
//! 
//! To easily encode and decode images from the [`image`][image-rs] crate, please enable the
//! optional dependency `"image"` of this crate in your `Cargo.toml`. Then you can use the
//! functions [`decompress_image()`][crate::decompress_image] and
//! [`compress_image()`][crate::compress_image]:
//! 
//! ```
//! // read JPEG data from file
//! let jpeg_data = std::fs::read("examples/parrots.jpg")?;
//!
//! // decompress `jpeg_data` into an `image::RgbImage`
//! let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data)?;
//!
//! // compress `image` into JPEG with quality 95 and 2x2 chrominance subsampling
//! let jpeg_data = turbojpeg::compress_image(&image, 95, turbojpeg::Subsamp::Sub2x2)?;
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! 
//! This crate supports these specializations of [`image::ImageBuffer`][::image::ImageBuffer]:
//! 
//! - [`image::RgbImage`][::image::RgbImage]
//! - [`image::RgbaImage`][::image::RgbaImage] (JPEG does not support alpha channel, so alpha is
//!   ignored when encoding and set to 255 when decoding)
//! - [`image::GrayImage`][::image::GrayImage]
//! 
//! [image-rs]: https://docs.rs/image/*/image/index.html
//!
//! # The [`Image`] type
//!
//! For more advanced usage of TurboJPEG, you will need to use the [`Image`] type. This is a simple
//! struct that contains the geometry of the image (width, height and pitch/stride), pixel format
//! (such as RGB, ABGR or grayscale) and the pixel data itself.
//!
//! [`Image`] is parameterized by the pixel data container, so you will use `Image<&[u8]>` as input
//! argument for compression, `Image<&out [u8]>` as output argument for decompression, and you may
//! also find `Image<Vec<u8>>` useful as an owned container of image data in you application.
//!
//! # Operations
//!
//! - **Decompress** images from JPEG using [`decompress()`] or [`Decompressor`].
//! - **Compress** images into JPEG using [`compress()`] or [`Compressor`].
//! - **Transform** images without recompression using [`transform()`] or [`Transformer`]. The
//! transformations are described in the [`Transform`] struct.
//! - **Read header** of JPEG image to get its size without decompression using
//! [`Decompressor::read_header()`] or [`read_header()`].
//! 
//! # The [`OutputBuf`] and [`OwnedBuf`] types
//!
//! During decompression, we need to write the produced JPEG data into some memory buffer. You have
//! two options:
//!
//! - Write the data into a mutable slice (`&mut [u8]`) that you already allocated and initialized.
//! This has the disadvantage that you must allocate all memory up front, so you need to make the
//! buffer very large to ensure that it can hold the compressed image even in the worst case, when
//! the compression does not reduce the image size at all. You will also need to initialize the
//! memory to comply with the Rust safety requirements.
//!
//! - Write the data into a memory buffer managed by TurboJPEG. This has the advantage that
//! TurboJPEG can automatically resize the buffer, so we don't have to conservatively allocate and
//! initialize a large chunk of memory, but we can let TurboJPEG grow the buffer as needed. This
//! kind of buffer is exposed as the [`OwnedBuf`].
//!
//! To handle both of these cases, this crate provides the [`OutputBuf`] type, which can hold
//! either a `&mut [u8]` or an `OwnedBuf`.
//!
//! # Features
//!
//! - `image`: enables the optional dependency on the [`image`][image-rs] crate.
//! - `pkg-config`: uses pkg-config to find the `libturbojpeg` library.
//! - `bindgen`: uses [bindgen] to generate the `libturbojpeg` bindings.
//!
//! [bindgen]: https://rust-lang.github.io/rust-bindgen/
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub extern crate turbojpeg_sys as raw;
pub extern crate libc;

mod buf;
mod common;
mod compress;
mod decompress;
mod image;
mod transform;
pub use self::buf::{OwnedBuf, OutputBuf};
pub use self::common::{PixelFormat, Subsamp, Colorspace, Result, Error};
pub use self::compress::{Compressor, compress, compressed_buf_len};
pub use self::decompress::{Decompressor, DecompressHeader, decompress, read_header};
pub use self::image::Image;
pub use self::transform::{Transformer, Transform, TransformOp, TransformCrop, transform};

#[cfg(feature = "image")]
mod image_rs;
#[cfg(feature = "image")]
pub use self::image_rs::{JpegPixel, compress_image, decompress_image};

