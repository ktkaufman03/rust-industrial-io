// libiio-sys/build.rs
//
// The builder for the Linux Industrial I/O wrapper crate.
//
// Copyright (c) 2018-2022, Frank Pagliughi
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use std::{
    env,
    path::{Path, PathBuf},
};

fn pass_env_as_cmake_define(env_name: &'static str, config: &mut cmake::Config) {
    if let Ok(env_val) = std::env::var(env_name) {
        config.define(env_name, env_val);
    }
}

fn main() {
    let libiio_path = Path::new("vendor/libiio");
    let mut config = cmake::Config::new(libiio_path);
    config.define("INSTALL_UDEV_RULE", "OFF");
    pass_env_as_cmake_define("LIBUSB_INCLUDE_DIR", &mut config);
    pass_env_as_cmake_define("LIBUSB_LIBRARIES", &mut config);

    let dst = config.build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=iio");

    let bindings = bindgen::Builder::default()
        .header(dst.join("include/iio.h").to_string_lossy())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("iio_bindings.rs"))
        .expect("Couldn't write bindings!");
}
