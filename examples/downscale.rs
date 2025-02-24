fn main() -> Result<(), Box<dyn std::error::Error>> {
    use turbojpeg::{Decompressor, Image, PixelFormat, ScalingFactor};

    // get the JPEG data
    let jpeg_data = std::fs::read("image.jpg")?;

    // initialize a Decompressor
    let mut decompressor = Decompressor::new()?;

    // set the desired downscale factor
    let scale = ScalingFactor::OneQuarter;

    // read the JPEG header with image size
    let header = decompressor.read_header(&jpeg_data)?.with_scale(scale);

    println!("{},{}", header.width, header.height);

    // prepare the destination image
    let mut image = Image {
        pixels: vec![0; 3 * header.width * header.height],
        width: header.width,
        pitch: 3 * header.width, // we use no padding between rows
        height: header.height,
        format: PixelFormat::RGB,
    };

    // set downscale factor
    decompressor.set_scaling_factor(scale)?;

    // decompress the JPEG data
    decompressor.decompress(&jpeg_data, image.as_deref_mut())?;

    // use the raw pixel data
    println!("{:?}", &image.pixels[0..9]);

    // initialize a Compressor
    let mut compressor = turbojpeg::Compressor::new()?;

    compressor.set_quality(40)?;

    // compress the Image to a Vec<u8> of JPEG data
    let jpeg_data = compressor.compress_to_vec(image.as_deref())?;

    std::fs::write("image-downscaled.jpg", jpeg_data)?;
    Ok(())
}
