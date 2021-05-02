# rust-turbojpeg

Rust bindings for TurboJPEG, which provides simple and fast
compression/decompression of JPEG images.

TurboJPEG is a high-level API provided by [libjpeg-turbo].

[libjpeg-turbo]: https://libjpeg-turbo.org/


## Usage

Use `turbojpeg::Compressor` to compress raw pixel data into JPEG (see
`examples/simple_compress.rs` for full example):

```rust
use turbojpeg::{Compressor, Image, PixelFormat};

// prepare the raw pixel data
let width: usize = ...;
let height: usize = ...;
let pixels: Vec<u8> = ...;

// initialize a Compressor
let mut compressor = Compressor::new()?;

// create an Image that bundles a reference to the raw pixel data (as &[u8])
// with information about the image format
let image = Image {
    pixels: pixels.as_slice(),
    width: width,
    pitch: 3 * width, // there is no padding between rows
    height: height,
    format: PixelFormat::RGB,
};

// compress the Image to a Vec<u8> of JPEG data
let jpeg_data = compressor.compress_to_vec(image)?;
```

To decompress JPEG data into a raw pixel data, use `turbojpeg::Decompressor`
(full example in `examples/simple_decompress.rs`):

```rust
use turbojpeg::{Decompressor, Image, PixelFormat};

// get the JPEG data
let jpeg_data: &[u8] = ...;

// initialize a Decompressor
let mut decompressor = Decompressor::new()?;

// read the JPEG header with image size
let header = decompressor.read_header(jpeg_data)?;
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
decompressor.decompress_to_slice(jpeg_data, image)?;

// use the raw pixel data
println!("{:?}", &pixels[0..9]);
```

See other examples in `examples/` or [read the docs][docs] for more information.

[docs]: https://docs.rs/turbojpeg


## Requirements

The low-level binding to `libturbojpeg` is provided by the crate
`turbojpeg-sys`, which needs:

- C headers to generate the Rust binding code using [`bindgen`][bindgen].
- Linker flags that `rustc` will use to link against `libturbojpeg`.

By default, the `turbojpeg-sys` crate uses a pregenerated Rust binding code (so
you don't need the C headers) and the default linker flags `-l turbojpeg`.
However, this behavior can be altered in several ways:

- Feature flag `pkg-config` uses the `pkg-config` tool to find the linker flags
    and the include paths for C headers that are specific for your system.
- Environment variable `TURBOJPEG_INCLUDE_PATH`, if specified, adds an extra
    include path for C headers.
- Feature flag `bindgen` uses the `bindgen` tool to generate Rust binding code
    at build time, instead of using the pregenerated code. If no include paths
    are specified (using `pkg-config` or `TURBOJPEG_INCLUDE_PATH`), we use
    headers that are bundled with `turbojpeg-sys`.

All this magic is implemented in the `build.rs` script in `turbojpeg-sys`. If
you think that it could be improved, or if you encounter an error on your
system, please open an issue or a pull request.

[bindgen]: https://github.com/rust-lang/rust-bindgen


## Contributing

All contributions are welcome! Please contact me (@honzasp) or open a pull
request. This crate is rather minimal, the main areas of improvement are:

- Extending the safe Rust API provided by `turbojpeg` crate.
- Improving the build process of `turbojpeg-sys` crate, so that it works
    seamlessly on a wide range of systems.
- Testing.


## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.
