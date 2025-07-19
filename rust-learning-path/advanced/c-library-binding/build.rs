fn main() {
    // Compile the C library
    cc::Build::new()
        .file("c-lib/mathlib.c")
        .include("c-lib")
        .compile("mathlib");
    
    // Tell cargo to tell rustc to link the mathlib library
    println!("cargo:rustc-link-lib=static=mathlib");
    
    // Tell cargo to invalidate the built crate whenever the C source changes
    println!("cargo:rerun-if-changed=c-lib/mathlib.c");
    println!("cargo:rerun-if-changed=c-lib/mathlib.h");
}