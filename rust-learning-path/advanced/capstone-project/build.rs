use std::env;
use std::path::PathBuf;

fn main() {
    // Build the C library
    cc::Build::new()
        .file("c-lib/task_ops.c")
        .include("c-lib")
        .compile("task_ops");

    // Tell cargo to invalidate the built crate whenever the C source changes
    println!("cargo:rerun-if-changed=c-lib/task_ops.c");
    println!("cargo:rerun-if-changed=c-lib/task_ops.h");

    // Link the C library
    println!("cargo:rustc-link-lib=static=task_ops");
    
    // Add the library search path
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}