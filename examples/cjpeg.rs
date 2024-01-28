use std::fs;
use anyhow::{Result, Context as _};
use clap::clap_app;

use turbojpeg::{Compressor, Image, PixelFormat};

fn main() -> Result<()> {
    let args = clap_app!(cjpeg =>
        (about: "Compresses an image to JPEG")
        (@arg INPUT: <input> "Input image file")
        (@arg OUTPUT: <output> "Output JPEG file")
        (@arg QUALITY: -q --quality <quality> 
            "Quality of the output JPEG file (1 is worst, 100 is best)")
    ).get_matches();

    let image = image::io::Reader::open(args.value_of("INPUT").unwrap())?
        .with_guessed_format()?
        .decode()?
        .to_rgb8();
    let image_flat = image.as_flat_samples();
    let strides = image_flat.strides_cwh();
    let extents = image_flat.extents();
    assert_eq!(strides.0, 1);
    assert_eq!(strides.1, 3);
    assert_eq!(extents.0, 3);

    let mut compressor = Compressor::new()?;

    if let Some(quality) = args.value_of("QUALITY") {
        let quality = quality.parse().context("could not parse value of --quality")?;
        compressor.set_quality(quality)?;
    }

    let image_jpeg = compressor.compress_to_owned(Image {
        pixels: image_flat.as_slice(),
        width: extents.1,
        pitch: strides.2,
        height: extents.2,
        format: PixelFormat::RGB,
    })?;

    fs::write(args.value_of("OUTPUT").unwrap(), &image_jpeg)?;
    Ok(())
}
