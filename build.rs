// Config for the linker

use std::env;

fn main() {
    println!("cargo:rustc-link-search=native={}/lib", env::var("CARGO_MANIFEST_DIR").unwrap()); // compile-time linkage
    println!("cargo:rustc-link-arg=-Wl,--rpath={}/lib", env::var("CARGO_MANIFEST_DIR").unwrap()); // run-time linkage
}