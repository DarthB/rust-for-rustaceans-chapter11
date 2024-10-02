# Rust for Rustaceans - Chapter 11 - Foreign Function Interfaces

This Repository has been created for the Book Club at [Rust and C++ Dragons Meetup](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)

Rust for Rustaceans - Chapter 11 Foreign Function Interfaces

RECORDING: TBA

Slides in [2024-10-02_FFI.md](./2024-10-02_FFI.md) generated with MARP.

PDF of Slides at [2024.10-02.pdf](./2024-10-02_FFI.pdf)

## Agenda

- Crossing language boundaries
- Types across language boundaries
- Calling Rust from foreign languages, calling C from Rust 
- Small Example focus on getting a C API safely in Rust
- Conclusion with good crates from the Rust Ecosystem for more serious FFI endavours

## About the small Example

The idea is how to use ffi only with the module `std::ffi`. Good for learning but for serious endavours make sure to checkout the references to specialized tools for FFI.

The [build script](./build.rs) compiles the [C file lib.c](./src/lib.c) and links it as static library.

The Rust code in [main.rs](./src/main.rs) is divided in an `ffi` module and the main function, which is seperated in three parts:

1. Using the C API for generation, transformation and querying
2. Generating sub objects with liftime constraints
3. Implementing a Rust function that is set as callback for the C library

### Output
```
API Create, transform and Query functions
-------
Init in C=b36b1cc0, str=ABCDEFGHIJKLMNOPQRSTUVWXYZ
context is lowercase
context_to_lower called: C=b36b1cc0, str=abcdefghijklmnopqrstuvwxyz
context_to_upper called: C=b36b1cc0, str=ABCDEFGHIJKLMNOPQRSTUVWXYZ
context_to_upper called: C=b36b1cc0, str=!"#$%&'()*+,-./0123456789:
context is OUT-OF-SPEC

Subobject
---------
Init in C=b36b1ba0, str=ABCDEFGHIJKLMNOPQRSTUVWXYZ
create_so with offset=16 from C
default C callback
so: QRSTUVWXYZ
context_to_lower called: C=b36b1ba0, str=abcdefghijklmnopqrstuvwxyz
so: qrstuvwxyz

Callback
--------
create_so with offset=20 from C
Rust Callback for 'uvwxyz' invoked!
free_context in C=b36b1ba0
free_context in C=b36b1cc0
```

## FFI Crates from the Rust Ecosystem

- [CBindgen]() - generate C/C++11 headers for Rust libraries which expose a public C API. Either as command-line tool: `cbindgen --config cbindgen.toml --crate my_rust_library --output my_header.h` or as a library that can be used in `build.rs` build scripts.
- [CXX](https://cxx.rs/) - Rust to C++ and C++ to Rust - It carves out a regime of commonality where Rust and C++ are semantically very similar and guides the programmer to express their language boundary effectively within this regime.
- [PyO3](https://docs.rs/pyo3/latest/pyo3/) can be used to write native Python modules or run Python code and modules from Rust.

All this crates provide good documentation and examples. Check them out.

## References

- [The Book](https://www.amazon.de/-/en/Jon-Gjengset-ebook/dp/B0957SWKBS)
- [The path to a stable ABI for Rust](https://youtu.be/MY5kYqWeV1Q) - Talk by Amanieu D'Antras - on Rust Nation UK
- [Polars](https://pola.rs) - Data Frames in Python with Rust
- [OpEn](https://alphaville.github.io/optimization-engine/) - Optmization for Model-Predictive-Control uses Rust Code generation.
- `Mid-level Intermediate Representation` short: [MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html).
- [Interacting with Data from FFI in Rust](https://blog.guillaume-gomez.fr/articles/2021-07-29+Interacting+with+data+from+FFI+in+Rust) Blog Article by Guillame Gomez
- [Bitflags Crate](https://docs.rs/bitflags/latest/bitflags/)
- [Bevy Engine](https://bevyengine.org/) 
- [bevy_dylib](https://lib.rs/crates/bevy_dylib) (dynamic linking for Bevy)
- [Async Rust/C++ Interop](https://www.youtube.com/watch?v=tsEuA9S5q9Q) Talk by Aida Getoeva
- [Async Rust/C++ Interop](https://medium.com/@aidagetoeva/async-c-rust-interoperability-39ece4cd3dcf) Blog Article by Aida Getoeva
- [Rust Nomincon Chapter on FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [DARPA Program Tractor](https://www.darpa.mil/program/translating-all-c-to-rust) (Transform all C Code to Rust)
- [rust-bindgen GitHub](https://github.com/rust-lang/rust-bindgen) (Auto Generate Bindings from C to Rust)
- [CBindgen](https://github.com/mozilla/cbindgen/blob/master/docs.md) (Auto Generate Glue Code for Rust and C/C++ Interop)
- [CXX](https://cxx.rs/) (Generate Bindings from/to Rust especially for C++)
- [PyO3](https://github.com/PyO3/pyo3) Rust to/from Python also used in [Polars](https://pola.rs)
- [PyO3 User Guide](https://pyo3.rs/v0.22.3/getting-started)
- [Mangling Format v0](https://doc.rust-lang.org/rustc/symbol-mangling/v0.html)
- [RFC-2945](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html) - regarding `extern "C-unwind"`