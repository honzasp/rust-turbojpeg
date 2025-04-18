# Changelog

## Unreleased

- Actually upgrade libjpeg-turbo to version 3.1.0 ([#30](https://github.com/honzasp/rust-turbojpeg/issues/30))

## 1.3.1 -- 2025-04-16

- Upgrade libjpeg-turbo to version 3.1.0 ([#29](https://github.com/honzasp/rust-turbojpeg/pull/29))

## 1.3.0 -- 2025-03-10

- Add scaling during decompression ([#27](https://github.com/honzasp/rust-turbojpeg/pull/27))
  - Add `ScalingFactor`
  - Add `DecompressHeader::scaled()`
  - Add `Decompressor::set_scaling_factor()` and `Decompressor::scaling_factor()`
  - Add `Decompressor::supported_scaling_factors()`
- Don't pass `DESTDIR` env variable to CMake ([#28](https://github.com/honzasp/rust-turbojpeg/pull/28))

## 1.2.2 -- 2025-02-14

- Add `Compressor::set_lossless()` to enable lossless JPEG compression ([#26](https://github.com/honzasp/rust-turbojpeg/pull/26))

## 1.2.1 -- 2025-01-20

- Fix `Subsamp::from_int()` to handle `TJSAMP_411` ([#24](https://github.com/honzasp/rust-turbojpeg/pull/24))

## 1.2.0 -- 2025-01-16

- Add `Subsamp::Unknown` to handle unusual chrominance subsampling options
  ([#23](https://github.com/honzasp/rust-turbojpeg/pull/23))

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
