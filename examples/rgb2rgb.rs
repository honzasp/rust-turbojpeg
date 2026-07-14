use std::path::PathBuf;

use turbojpeg::{Colorspace, Compressor, Decompressor};

fn get_image_rgb() -> turbojpeg::Image<Vec<u8>> {
    let jpeg_data = std::fs::read("examples/parrots.jpg").unwrap();
    eprintln!("Read {} bytes from examples/parrots.jpg", jpeg_data.len());
    let out = turbojpeg::decompress(&jpeg_data, turbojpeg::PixelFormat::RGB).unwrap();
    eprintln!("Decompressed to {} bytes", out.pixels.len());
    out
}

fn write_tmp(fname: &str, data: impl AsRef<[u8]>) {
    let tmppath = PathBuf::from("tmp");
    if !std::fs::exists(&tmppath).unwrap() {
        std::fs::create_dir(&tmppath).unwrap();
    }
    let outpath = tmppath.join(fname);
    std::fs::write(&outpath, &data).unwrap();
    eprintln!(
        "Wrote {} bytes to {}",
        data.as_ref().len(),
        outpath.display()
    );
}

fn main() {
    let image = get_image_rgb();

    let mut comp = Compressor::new().unwrap();
    comp.set_colorspace(Colorspace::RGB).unwrap();
    let owned = comp.compress_to_owned(image.as_deref()).unwrap();
    let hdr = Decompressor::new().unwrap().read_header(&owned).unwrap();
    eprintln!("{hdr:?}");

    write_tmp("parrots_rgb.jpeg", &owned);
}
