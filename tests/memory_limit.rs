use turbojpeg::{Compressor, Decompressor, Error, Image, PixelFormat, ScalingFactor, Subsamp};

#[test]
fn reduced_progressive_decode_obeys_memory_limit_and_can_recover() -> turbojpeg::Result<()> {
    let source = Image {
        pixels: vec![128; 1024 * 1024 * 3],
        width: 1024,
        pitch: 1024 * 3,
        height: 1024,
        format: PixelFormat::RGB,
    };
    let mut compressor = Compressor::new()?;
    compressor.set_subsamp(Subsamp::None)?;
    compressor.set_progressive(true)?;
    let jpeg = compressor.compress_to_vec(source.as_deref())?;
    let mut output = Image {
        pixels: vec![0; 128 * 128 * 3],
        width: 128,
        pitch: 128 * 3,
        height: 128,
        format: PixelFormat::RGB,
    };
    let mut decoder = Decompressor::new()?;
    decoder.set_scaling_factor(ScalingFactor::ONE_EIGHTH)?;
    decoder.set_max_memory(1)?;
    assert!(decoder.decompress(&jpeg, output.as_deref_mut()).is_err());
    decoder.set_max_memory(64)?;
    decoder.decompress(&jpeg, output.as_deref_mut())?;
    let expected = output.pixels.clone();
    output.pixels.fill(0);
    decoder.set_max_memory(0)?;
    decoder.decompress(&jpeg, output.as_deref_mut())?;
    assert_eq!(output.pixels, expected);
    Ok(())
}

#[test]
fn memory_limit_rejects_values_that_overflow_the_native_api() -> turbojpeg::Result<()> {
    let mut decoder = Decompressor::new()?;
    assert!(matches!(decoder.set_max_memory(u32::MAX), Err(Error::IntegerOverflow(_))));
    Ok(())
}
