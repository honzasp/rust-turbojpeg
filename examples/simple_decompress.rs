fn main() -> Result<(), Box<dyn std::error::Error>> {
    use turbojpeg::{Decompressor, Image, PixelFormat};

    // get the JPEG data
    let jpeg_data = std::fs::read("image.jpg")?;

    // initialize a Decompressor
    let mut decompressor = Decompressor::new()?;

    // read the JPEG header with image size
    let header = decompressor.read_header(&jpeg_data)?;
    let (width, height) = (header.width, header.height);

    // prepare a storage for the raw pixel data
    let mut pixels = vec![0; 3*width*height];
    let image = Image {
        pixels: pixels.as_mut_slice(),
        width: width,
        pitch: 3 * width, // we use no padding between rows
        height: height,
        format: PixelFormat::RGB,
    };

    // decompress the JPEG data 
    decompressor.decompress_to_slice(&jpeg_data, image)?;

    // use the raw pixel data
    println!("{:?}", &pixels[0..9]);
    Ok(())
}
