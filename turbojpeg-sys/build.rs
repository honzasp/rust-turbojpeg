#![allow(unused_imports, dead_code)]
use anyhow::{Result, Context as _, anyhow};
use std::{env, fs};
use std::path::PathBuf;

fn main() -> Result<()> {
    #[cfg(feature = "bindgen")]
    {
        println!("building bindings with bindgen");
        generate_bindings()?;
    }

    #[cfg(not(feature = "bindgen"))]
    {
        println!("using pregenerated bindings");
        copy_pregenerated_bindings()?;
    }

    #[cfg(feature = "pkg-config")]
    {
        find_pkg_config()?;
    }

    #[cfg(not(feature = "pkg-config"))]
    {
        println!("cargo:rustc-flags=-l turbojpeg");
    }

    Ok(())
}

#[cfg(feature = "bindgen")]
fn generate_bindings() -> Result<()> {
    let target = env::var("TARGET").unwrap();
    let include_paths = find_include_paths()?;

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .ctypes_prefix("libc")
        .clang_args(&["-target", &target]);

    for path in include_paths.iter() {
        let path = path.to_str().unwrap();
        builder = builder.clang_arg(format!("-I{}", path));
        println!("cargo:rerun-if-changed={}", path);
    }

    let bindings = builder.generate()
        .map_err(|_| anyhow!("could not generate bindings"))?;

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .context("could not write bindings to OUT_DIR")?;

    Ok(())
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() -> Result<()> {
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(crate_path.join("bindings.rs"), out_path.join("bindings.rs"))?;
    println!("cargo:rerun-if-changed={}", crate_path.join("bindings.rs").to_str().unwrap());
    Ok(())
}

fn find_include_paths() -> Result<Vec<PathBuf>> {
    let mut include_paths = vec![];

    if let Some(path) = env::var_os("TURBOJPEG_INCLUDE_PATH") {
        println!("using TURBOJPEG_INCLUDE_PATH = {:?}", path);
        include_paths.push(path.into());
    }
    println!("cargo:rerun-if-env-changed=TURBOJPEG_INCLUDE_PATH");

    #[cfg(feature = "pkg-config")]
    {
        let library = find_pkg_config()?;
        println!("using pkg-config include paths: {:?}", library.include_paths);
        include_paths.extend(library.include_paths.into_iter());
    }

    if include_paths.is_empty() {
        let mut path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
        path.push("libjpeg-turbo-20.0.9");
        println!("using the bundled headers: {:?}", path);
        include_paths.push(path);
    }

    Ok(include_paths)
}

#[cfg(feature = "pkg-config")]
fn find_pkg_config() -> Result<pkg_config::Library> {
    pkg_config::Config::new()
        .atleast_version("2.0")
        .probe("libturbojpeg")
        .context("could not find turbojpeg using pkg-config")
}
