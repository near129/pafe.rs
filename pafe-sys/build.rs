use std::env;
use std::path::PathBuf;

#[cfg(feature = "vendored")]
fn build(libdir_path: &PathBuf, out_dir: &str) {
    let tmp_libdir_path = PathBuf::from(out_dir).join("libpafe");
    copy_dir::copy_dir(libdir_path.clone(), tmp_libdir_path.clone()).unwrap();
    if !std::process::Command::new(tmp_libdir_path.join("configure"))
        .arg("--build")
        .arg(env::var("HOST").unwrap())
        .arg("--host")
        .arg(env::var("TARGET").unwrap())
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
    let out_dir = env::var("OUT_DIR").unwrap();
    let libdir_path = PathBuf::from("libpafe")
        .canonicalize()
        .expect("cannot canonicalize path");
    let headers_path = libdir_path.join("src/libpafe.h");

    println!("cargo:rustc-link-lib=libpafe");
    println!("cargo:rustc-link-lib=dylib=usb-1.0");

    #[cfg(feature = "vendored")]
    {
        build(&libdir_path, &out_dir);
        println!("cargo:rustc-link-search={}", out_dir);
    }

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
