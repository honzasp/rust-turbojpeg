fn main() -> Result<(), Box<dyn std::error::Error>> {
    use turbojpeg::{Compressor, Image, PixelFormat};

    // prepare the pixel data
    let width = 768;
    let height = 512;
    let mut pixels = vec![0; 3*width*height];
    for y in 0..height {
        for x in 0..width {
            let r = if (x/32 + y/32) % 2 == 0 { 0 } else { 255 };
            let g = 255 - (x * 255 / width) as u8;
            let b = (y * 255 / height) as u8;
            pixels[3*width*y + 3*x + 0] = r;
            pixels[3*width*y + 3*x + 1] = g;
            pixels[3*width*y + 3*x + 2] = b;
        }
    }

    // initialize a Compressor
    let mut compressor = Compressor::new()?;

    // create an Image that bundles a reference to the raw pixel data (as &[u8]) with information
    // about the image format
    let image = Image {
        pixels: pixels.as_slice(),
        width: width,
        pitch: 3 * width, // there is no padding between rows
        height: height,
        format: PixelFormat::RGB,
    };

    // compress the Image to a Vec<u8> of JPEG data
    let jpeg_data = compressor.compress_to_vec(image)?;

    std::fs::write("image.jpg", jpeg_data)?;
    Ok(())
}

