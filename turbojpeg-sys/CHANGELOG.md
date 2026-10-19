# Changelog

## Unreleased

- Build libjpeg-turbo with the same C runtime as Rust on MSVC targets, which fixes
  `LNK4098: defaultlib 'LIBCMTD' conflicts with use of other libs` in debug builds

## 1.2.0

- Set `DEP_TURBOJPEG_INCLUDE` in build scripts of dependent packages
  ([#37](https://github.com/honzasp/rust-turbojpeg/pull/37))

## 1.1.1 -- 2025-04-18

- Upgrade to libjpeg-turbo version 3.1.0

## 1.0.1 -- 2024-09-29

- Fix compilation on `x86_64-pc-windows-gnu` target ([#22](https://github.com/honzasp/rust-turbojpeg/pull/22))

## 1.0.0 -- 2024-02-03

- Upgrade to TurboJPEG 3.0.1 ([#15](https://github.com/honzasp/rust-turbojpeg/pull/15))
