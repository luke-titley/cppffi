//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
mod arguments;
mod class_info;
mod ffi_expose;
mod gen;
mod generator;
mod result;
mod state;
mod supported_types;
mod utils;

//------------------------------------------------------------------------------
fn main() {
    generator::run(
        &["expose.h"],
        "c_tests/usd_c.h",
        "c_tests/usd_c.cpp",
        &[
        "-x","c++", "-std=c++14",
         "-isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk",
         "-isystem/Library/Developer/CommandLineTools/usr/include/c++/v1/",
         "-isystem/Library/Developer/CommandLineTools/usr/lib/clang/10.0.1/include",
          "-I/Volumes/src/usd-rs/usd-rs/thirdparty/docs/include"],
    );
}
