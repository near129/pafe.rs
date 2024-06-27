use std::env;
use std::path::PathBuf;

#[cfg(feature = "vendored")]
fn build(libdir_path: &PathBuf, out_dir: &PathBuf) {
    let tmp_libdir_path = out_dir.join("libpafe");
    if tmp_libdir_path.exists() {
        std::fs::remove_dir_all(&tmp_libdir_path).unwrap();
    }
    copy_dir::copy_dir(&libdir_path, &tmp_libdir_path).unwrap();
    std::fs::copy("prepare_build.sh", out_dir.join("prepare_build.sh")).unwrap();
    if !std::process::Command::new("./prepare_build.sh")
        .current_dir(&out_dir)
        .status()
        .unwrap()
        .success()
    {
        panic!("failed to run prepare_build.sh");
    }
    if !std::process::Command::new(tmp_libdir_path.join("configure"))
        .arg("--host")
        .arg(format!(
            "{}-{}-{}",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ENV").unwrap()
        ))
        .arg("--prefix")
        .arg(&out_dir)
        .current_dir(&tmp_libdir_path)
        .status()
        .unwrap()
        .success()
    {
        panic!("failed to run configure");
    }
    if !std::process::Command::new("make")
        .current_dir(&tmp_libdir_path)
        .status()
        .unwrap()
        .success()
    {
        panic!("failed to run make");
    }
    if !std::process::Command::new("make")
        .arg("install")
        .current_dir(&tmp_libdir_path)
        .status()
        .unwrap()
        .success()
    {
        panic!("failed to run make");
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .canonicalize()
        .expect("cannot canonicalize path");
    let libdir_path = PathBuf::from("libpafe")
        .canonicalize()
        .expect("cannot canonicalize path");
    let headers_path = libdir_path.join("src/libpafe.h");

    #[cfg(feature = "vendored")]
    {
        build(&libdir_path, &out_dir);
        println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    }

    println!("cargo:rustc-link-lib=pafe");
    println!("cargo:rustc-link-lib=dylib=usb-1.0");

    let bindings = bindgen::Builder::default()
        .header(headers_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
