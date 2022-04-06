# rust-turbojpeg

Rust bindings for [TurboJPEG][libjpeg-turbo], which provides simple and fast
operations for JPEG images:

- Compression (encoding)
- Decompression (decoding)
- Lossless transformations

[libjpeg-turbo]: https://libjpeg-turbo.org/

## Usage with `image-rs`

To quickly encode and decode images from the [`image`][image-rs] crate, add this
to the `[dependencies]` section in your `Cargo.toml`:

    turbojpeg = {version = "0.4", features = ["image"]}

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
`turbojpeg-sys`, which needs:

- Rust binding code generated from C headers using [`bindgen`][bindgen].
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

- Improving the build process of `turbojpeg-sys` crate, so that it works
    seamlessly on a wide range of systems.
- Testing.
- Extending the safe Rust API provided by `turbojpeg` crate.

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.
