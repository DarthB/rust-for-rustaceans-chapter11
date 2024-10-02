fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/lib.c");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new().file("src/lib.c").compile("lib_in_c");

    // Here is also the place where tool like CBindgen get included
    // - CBindgen generates Rust FFI code from header files,
    // see: https://github.com/mozilla/cbindgen
}
