fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create an image
    let (width, height) = (400, 300);
    let image_1 = image::RgbImage::from_fn(width, height, |x, y| {
        let r = if (x/32 + y/32) % 2 == 0 { 0 } else { 255 };
        let g = 255 - (x * 255 / width) as u8;
        let b = (y * 255 / height) as u8;
        image::Rgb([r, g, b])
    });

    // compress the image into JPEG
    let jpeg_data = turbojpeg::compress_image(&image_1, 95, turbojpeg::Subsamp::None)?;
    println!("compressed into {} bytes", jpeg_data.len());

    // decompress the JPEG into image
    let image_2: image::RgbImage = turbojpeg::decompress_image(&jpeg_data)?;

    assert_eq!(image_1.width(), image_2.width());
    assert_eq!(image_1.height(), image_2.height());
    let error_sum = image_1.pixels().zip(image_2.pixels())
        .map(|(pix_1, pix_2)| (0..3).map(move |i| pix_1[i] as i64 - pix_2[i] as i64))
        .flatten()
        .map(|diff| diff * diff)
        .sum::<i64>();
    let error_mean = error_sum as f64 / (image_1.width() * image_1.height()) as f64;
    println!("mean squared error: {:.1}", error_mean);

    Ok(())
}
