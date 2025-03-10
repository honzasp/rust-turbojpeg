use std::fs;
use anyhow::{Result, Context as _};
use clap::clap_app;

use turbojpeg::{Decompressor, Image, PixelFormat};

fn main() -> Result<()> {
    let args = clap_app!(djpeg =>
        (about: "Decompresses an image from JPEG")
        (@arg INPUT: <input> "Input JPEG file")
        (@arg OUTPUT: <output> "Output image file")
        (@arg SCALE: -s --scale [scale] "Apply a scaling factor (such as 7/8)")
    ).get_matches();

    let image_jpeg = fs::read(args.value_of("INPUT").unwrap())?;

    let scaling = match args.value_of("SCALE") {
        Some(scale) => {
            let (num, denom) = scale.split_once('/')
                .context("Wrong syntax of scale")?;
            let num = num.parse()?;
            let denom = denom.parse()?;
            turbojpeg::ScalingFactor::new(num, denom)
        },
        None => turbojpeg::ScalingFactor::ONE,
    };

    let mut decompressor = Decompressor::new()?;
    decompressor.set_scaling_factor(scaling)?;

    let header = decompressor.read_header(&image_jpeg)?;
    let scaled = header.scaled(scaling);

    let mut image = image::RgbImage::new(scaled.width as u32, scaled.height as u32);
    let mut image_flat = image.as_flat_samples_mut();
    let strides = image_flat.strides_cwh();
    let extents = image_flat.extents();
    assert_eq!(strides.0, 1);
    assert_eq!(strides.1, 3);
    assert_eq!(extents.0, 3);

    decompressor.decompress(&image_jpeg, Image {
        pixels: image_flat.as_mut_slice(),
        width: extents.1,
        pitch: strides.2,
        height: extents.2,
        format: PixelFormat::RGB,
    })?;

    image.save(args.value_of("OUTPUT").unwrap())?;
    Ok(())
}
