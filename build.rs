/*
 * File: build.rs
 * Project: bindgen-test
 * Created Date: 22/03/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/03/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cpp/sample.hpp");
    println!("cargo:rerun-if-changed=cpp/sample.cpp");

    cc::Build::new()
        .warnings(true)
        .cpp(true)
        .static_flag(true)
        .file("cpp/sample.cpp")
        .include("cpp")
        .compile("sample");

    let bindings = bindgen::Builder::default()
        .header("cpp/sample.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("sample.rs"))
        .expect("Couldn't write bindings!");
}
