use std::fs;
use anyhow::{Result, Context as _, bail};
use clap::clap_app;

use turbojpeg::{Transform, TransformOp, Transformer};

fn main() -> Result<()> {
    let args = clap_app!(jpegtran =>
        (about: "Losslessly transform a JPEG image")
        (@arg INPUT: <input> "Input image file")
        (@arg OUTPUT: <output> "Output image file")

        (@arg FLIP: --flip [direction]
            "Performs a flip ('horizontal' or 'vertical')")
        (@arg ROTATE: --rotate [angle]
            "Rotates the image (angle is 90, 180 or 270)")
        (@arg TRANSPOSE: --transpose ...
            "Transpose image (flip along upper left to lower right axis)")
        (@arg TRANSVERSE: --transverse ...
            "Transverse transpose image (flip along upper right to lower left axis)")

        (@arg PERFECT: --perfect ...
            "Return an error if the transformation is not perfecly lossless")
        (@arg TRIM: --trim ...
            "Discard partial blocks that cannot be transformed")
        (@arg PROGRESSIVE: --progressive ...
            "Use progressive entropy coding")
        (@arg OPTIMIZE: --optimize ...
            "Use optimized baseline entropy coding")
        (@arg GRAYSCALE: --grayscale ...
            "Convert the image into grayscale")
        (@arg COPY_NONE: --("copy-none") ...
            "Do not copy any extra markers (such as EXIF data)")
    ).get_matches();

    let mut transform = Transform::default();
    if let Some(direction) = args.value_of("FLIP") {
        if direction == "horizontal" {
            transform.op = TransformOp::Hflip;
        } else if direction == "vertical" {
            transform.op = TransformOp::Vflip;
        } else {
            bail!("unknown value of --flip")
        }
    } else if let Some(angle) = args.value_of("ROTATE") {
        if angle == "90" {
            transform.op = TransformOp::Rot90;
        } else if angle == "180" {
            transform.op = TransformOp::Rot180;
        } else if angle == "270" {
            transform.op = TransformOp::Rot270;
        } else {
            bail!("unknown value of --rotate")
        }
    } else if args.is_present("TRANSPOSE") {
        transform.op = TransformOp::Transpose;
    } else if args.is_present("TRANSVERSE") {
        transform.op = TransformOp::Transverse;
    }

    transform.perfect = args.is_present("PERFECT");
    transform.trim = args.is_present("TRIM");
    transform.progressive = args.is_present("PROGRESSIVE");
    transform.gray = args.is_present("GRAYSCALE");
    transform.copy_none = args.is_present("COPY_NONE");

    // TODO: crop

    let jpeg_data = fs::read(args.value_of("INPUT").unwrap())
        .context("could not read input image")?;
    let mut transformer = Transformer::new()
        .context("could not create transformer")?;
    let transformed_data = transformer.transform_to_owned(&transform, &jpeg_data)
        .context("could not transform JPEG data")?;
    fs::write(args.value_of("OUTPUT").unwrap(), &transformed_data)
        .context("could not write output image")?;

    Ok(())
}
