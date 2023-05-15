// Config for the linker

use std::env;

fn main() {
    println!("cargo:rustc-link-search=native={}/libs", env::var("PWD").unwrap()); // compile-time linkage
    println!("cargo:rustc-link-arg=-Wl,--rpath={}/libs", env::var("PWD").unwrap()); // run-time linkage
}