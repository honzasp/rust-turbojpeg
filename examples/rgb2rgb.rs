use turbojpeg::{Colorspace, Compressor, Decompressor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jpeg_data_in = std::fs::read("examples/parrots.jpg")?;
    println!("Read {} bytes from examples/parrots.jpg", jpeg_data_in.len());
    let image = turbojpeg::decompress(&jpeg_data_in, turbojpeg::PixelFormat::RGB)?;
    println!("Decompressed to {} pixels", image.pixels.len());

    let mut comp = Compressor::new()?;
    comp.set_colorspace(Colorspace::RGB)?;
    let jpeg_data_out = comp.compress_to_owned(image.as_deref())?;

    let header = Decompressor::new()?.read_header(&jpeg_data_out)?;
    println!("Compressed to {} bytes: {:?}", jpeg_data_out.len(), header);

    std::fs::write("examples/rgb.tmp.jpg", jpeg_data_out)?;
    Ok(())
}
