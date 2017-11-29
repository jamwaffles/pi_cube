extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ws2811");
    println!("cargo:rustc-link-search=ws2811");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./wrapper.h")

        .whitelisted_type("ws2811_t")
        .whitelisted_type("ws2811_device")
        .whitelisted_type("ws2811_channel_t")
        .whitelisted_var("WS2811_STRIP_RGB")

        .whitelisted_function("ws2811_init")
        .whitelisted_function("ws2811_render")
        .whitelisted_function("rpi_hw_detect")
        .whitelisted_function("ws2811_fini")

        .clang_arg("-Iws2811")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}