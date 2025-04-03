use std::process::Command;
use std::env;
use std::path::Path;

// Compile
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mock_math_lib = Path::new(&root_dir).join("mock-math-lib");
    let wasm_lib = Path::new(&root_dir).join("wasm-lib");

    // Build mock_math_lib
    let status1 = Command::new("cargo")
        .args(&["build", "--target", "wasm32-unknown-unknown", "--release"])
        .current_dir(&mock_math_lib)
        .status()
        .expect("Failed to execute cargo build for mock_math_lib");

    if !status1.success() {
        panic!("Failed to build mock_math_lib");
    }

    // Build wasm_lib
    let status2 = Command::new("cargo")
        .args(&["build", "--target", "wasm32-unknown-unknown", "--release"])
        .current_dir(&wasm_lib)
        .status()
        .expect("Failed to execute cargo build for wasm_lib");

    if !status2.success() {
        panic!("Failed to build wasm_lib");
    }

    // Optional: Copy the WASM files to a desired output directory
    // Example: Copy to the main project's target/wasm directory
    let wasm_out_dir = Path::new(&out_dir).join("../../../target/wasm");
    std::fs::create_dir_all(&wasm_out_dir).unwrap();

    let mock_math_lib_wasm = mock_math_lib.join("target/wasm32-unknown-unknown/release/mock_math_lib.wasm");
    let wasm_lib_wasm = wasm_lib.join("target/wasm32-unknown-unknown/release/wasm_lib.wasm");

    let copy_mock_math_lib = std::fs::copy(&mock_math_lib_wasm, wasm_out_dir.join("mock_math_lib.wasm"));
    let copy_wasm_lib = std::fs::copy(&wasm_lib_wasm, wasm_out_dir.join("wasm_lib.wasm"));

    if copy_mock_math_lib.is_err() {
        println!("Failed to copy mock_math_lib.wasm");
    }
    if copy_wasm_lib.is_err() {
        println!("Failed to copy wasm_lib.wasm");
    }

    // Indicate that the build script generated some files that should be considered
    // for rebuilding when changed.
    println!("cargo:rerun-if-changed=mock_math_lib/src");
    println!("cargo:rerun-if-changed=wasm_lib/src");
}