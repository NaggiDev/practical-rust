//! Build script for the compiler plugin
//!
//! This build script handles any special compilation requirements
//! for the compiler plugin.

fn main() {
    // Check if we're building with nightly Rust
    let version = rustc_version::version().unwrap();
    if version.pre.is_empty() {
        println!("cargo:rustc-cfg=stable_rust");
    } else {
        println!("cargo:rustc-cfg=nightly_rust");
    }
    
    // Enable unstable features if building with nightly
    if std::env::var("CARGO_CFG_NIGHTLY_RUST").is_ok() {
        println!("cargo:rustc-cfg=feature=\"unstable\"");
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}