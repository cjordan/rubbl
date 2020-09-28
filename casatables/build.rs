// Copyright 2017-2020 Peter Williams <peter@newton.cx> and collaborators
// Licensed under the MIT License.

use std::env;

const FILES: &[&str] = &["src/glue.cc"];

fn main() {
    let mut builder = cc::Build::new();

    builder
        .cpp(true)
        .warnings(true)
        .flag_if_supported("-std=c++11")
        .include("src")
        .include(env::var_os("DEP_CASA_INCLUDE").unwrap())
        .files(FILES)
        .compile("libcasatables_glue.a");

    for file in FILES {
        println!("cargo:rerun-if-changed={}", file);
    }
}
