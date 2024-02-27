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

use std::{env, path::Path};

#[cfg(target_os = "macos")]
fn config_macos() {
    todo!();
    // println!("cargo:rustc-link-lib=framework=iio");

    // if cfg!(target_arch = "x86_64") {
    //     println!(r"cargo:rustc-link-search=framework=/usr/local/Frameworks/");
    // }
    // else {
    //     println!(r"cargo:rustc-link-search=framework=/opt/homebrew/Frameworks/");
    // }
}

fn main() {
    // TODO: We should eventually find or regenerate the
    //      bindings file for the specific target.
    // let tgt = env::var("TARGET").unwrap();
    // println!("debug: Building for target: '{}'", tgt);

    // #[cfg(not(target_os = "macos"))]
    // println!("cargo:rustc-link-lib=iio");

    let libiio_path = Path::new("vendor/libiio");
    let libiio_config_path = libiio_path.join("iio-config.h");
    std::fs::write(
        libiio_config_path,
        "
    #ifndef IIO_CONFIG_H
    #define IIO_CONFIG_H
    
    #define LIBIIO_VERSION_MAJOR	0
    #define LIBIIO_VERSION_MINOR	25
    #define LIBIIO_VERSION_GIT	\"29107ada\"
    
    #define LOG_LEVEL Info_L
    
    #define LIBIIO_SCAN_BACKENDS	\"\"
    
    #define WITH_LOCAL_BACKEND 1
    #define WITH_XML_BACKEND 0
    #define WITH_NETWORK_BACKEND 0
    #define WITH_USB_BACKEND 0
    #define WITH_SERIAL_BACKEND 0
    
    /* #undef WITH_NETWORK_GET_BUFFER */
    #define WITH_NETWORK_EVENTFD 0
    #define WITH_IIOD_USBD 0
    #define WITH_IIOD_SERIAL 0
    #define WITH_LOCAL_CONFIG 1
    #define WITH_LOCAL_MMAP_API 0
    #define WITH_HWMON 0
    #define WITH_AIO 0
    #define HAVE_DNS_SD 0
    #define HAVE_AVAHI 0
    #define WITH_ZSTD 0
    
    /* #undef HAS_PIPE2 */
    #define HAS_STRDUP
    /* #undef HAS_STRNDUP */
    #define HAS_STRTOK_R
    /* #undef HAS_STRERROR_R */
    /* #undef HAS_NEWLOCALE */
    /* #undef HAS_PTHREAD_SETNAME_NP */
    #define HAVE_IPV6
    /* #undef NO_THREADS */
    /* #undef HAS_LIBUSB_GETVERSION */
    
    #endif /* IIO_CONFIG_H */    
    ",
    )
    .unwrap();

    println!("debug: OUT_DIR={}", std::env::var("OUT_DIR").unwrap());
    cc::Build::new()
        .include(libiio_path)
        .files(
            vec![
                "backend.c",
                "channel.c",
                "device.c",
                "context.c",
                "buffer.c",
                "utilities.c",
                "scan.c",
                "sort.c",

                // local backend
                "local.c",
                "deps/libini/libini.c"
            ]
            .iter()
            .map(|x| libiio_path.join(x)),
        )
        .compile("iiotest");

    println!("cargo:rustc-link-lib=iiotest");

    // #[cfg(target_os = "macos")]
    // config_macos();
}
