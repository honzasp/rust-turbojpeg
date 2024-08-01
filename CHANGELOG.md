# Changelog

## 1.1.1 -- 2024-08-01

- Change `image` dependency to `>= 0.24, < 0.26` to support both 0.24 and 0.25
  ([#20](https://github.com/honzasp/rust-turbojpeg/pull/20))

## 1.1.0 -- 2024-04-14

- Add option for encoding YUV images ([#18](https://github.com/honzasp/rust-turbojpeg/pull/18))

## 1.0.2 -- 2024-04-07

- Reexport the `image` dependency ([#17](https://github.com/honzasp/rust-turbojpeg/pull/17))

## 1.0.1 -- 2024-03-17

- Fix the version requirement on `image` dependency to `^0.24`

## 1.0.0 -- 2024-02-03

- Upgrade to TurboJPEG 3.0.1 ([#15](https://github.com/honzasp/rust-turbojpeg/pull/15))
- Use the new TurboJPEG API (functions starting with `tj3`)
- Change `Error::Null()` to more conventional `Error::Null`
- Add `Subsamp::Sub1x4` for 4:4:1 chrominance subsampling
- Add `Compressor::set_optimize()` and `Transform::optimize` to enable optimized
  baseline entropy coding
- Add `#[non_exhaustive]` attribute to `Error`, `Subsamp`, `DecompressHeader`,
  `Transform` and `TransformOp`
- `Compressor::set_quality()` and `Compressor::set_subsamp()` now return a `Result<()>`

## 0.5.4 -- 2023-07-31

- Added support for decompression into YUV
