fn main() {
    let curr_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-env=LD_LIBRARY_PATH={curr_dir}/qiskit_c_lib/dist/c/lib");
    println!("cargo:rustc-link-search={curr_dir}/qiskit_c_lib/dist/c/lib");
    println!("cargo:rustc-link-lib=qiskit");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/qiskit_ffi.rs")
        .expect("Couldn't write bindings!");
}
