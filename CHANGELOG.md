# Changelog

## Unreleased

- Upgrade to TurboJPEG 3.0.1 ([#15](https://github.com/honzasp/rust-turbojpeg/pull/15))
- Use the new TurboJPEG API (functions starting with `tj3`)
- Change `Error::Null()` to more conventional `Error::Null`
- Add `Subsamp::Sub1x4` for 4:4:1 chrominance subsampling
- Add `#[non_exhaustive]` attribute to `Error`, `Subsamp` and `DecompressHeader`
- `Compressor::set_quality()` and `Compressor::set_subsamp()` now return a `Result<()>`

## 0.5.4 -- 2023-07-31

- Added support for decompression into YUV
