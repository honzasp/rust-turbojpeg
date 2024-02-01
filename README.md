# turbojpeg

Rust bindings for [TurboJPEG][libjpeg-turbo], which provides simple and fast
operations for JPEG images:

- Compression (encoding)
- Decompression (decoding)
- Lossless transformations

[libjpeg-turbo]: https://libjpeg-turbo.org/

## Usage with `image-rs`

To quickly encode and decode images from the [`image`][image-rs] crate, add this
to the `[dependencies]` section in your `Cargo.toml`:

    turbojpeg = {version = "1.0", features = ["image"]}

and then use the functions [`turbojpeg::decompress_image`][decompress] and
[`turbojpeg::compress_image`][compress].

For more advanced usage without the `image` crate, please [see the
documentation][docs].

[image-rs]: https://docs.rs/image/*/image/index.html
[compress]: https://docs.rs/turbojpeg/*/turbojpeg/fn.compress_image.html
[decompress]: https://docs.rs/turbojpeg/*/turbojpeg/fn.decompress_image.html
[docs]: https://docs.rs/turbojpeg/

## Requirements

The low-level binding to `libturbojpeg` is provided by the crate
`turbojpeg-sys`, which needs to link to the C library. Typically, you will need
CMake, a C compiler and NASM to build the library from source, but see [its
README][sys-readme] for details.

[sys-readme]: https://github.com/honzasp/rust-turbojpeg/tree/master/turbojpeg-sys

## Contributing

All contributions are welcome! Please contact me (@honzasp) or open a pull
request. This crate is rather minimal, the main areas of improvement are:

- Improving the build process of `turbojpeg-sys` crate, so that it works
    seamlessly on a wide range of systems.
- Testing.
- Extending the safe Rust API provided by `turbojpeg` crate.

## License

This software is released into the public domain or is available with the MIT
license (your choice).
