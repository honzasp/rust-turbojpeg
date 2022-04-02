use std::fs;
use anyhow::Result;
use clap::clap_app;

use turbojpeg::{Decompressor, Image, PixelFormat};

fn main() -> Result<()> {
    let args = clap_app!(djpeg =>
        (about: "Decompresses an image from JPEG")
        (@arg INPUT: <input> "Input JPEG file")
        (@arg OUTPUT: <output> "Output image file")
    ).get_matches();

    let image_jpeg = fs::read(args.value_of("INPUT").unwrap())?;

    let mut decompressor = Decompressor::new()?;
    let header = decompressor.read_header(&image_jpeg)?;

    let mut image = image::RgbImage::new(header.width as u32, header.height as u32);
    let mut image_flat = image.as_flat_samples_mut();
    let strides = image_flat.strides_cwh();
    let extents = image_flat.extents();
    assert_eq!(strides.0, 1);
    assert_eq!(strides.1, 3);
    assert_eq!(extents.0, 3);

    decompressor.decompress_to_slice(&image_jpeg, Image {
        pixels: image_flat.as_mut_slice(),
        width: extents.1,
        pitch: strides.2,
        height: extents.2,
        format: PixelFormat::RGB,
    })?;

    image.save(args.value_of("OUTPUT").unwrap())?;
    Ok(())
}
