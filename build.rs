use std::env;
use std::path::Path;
use std::process::Command;

/// Executes the build process for the `mock-math-lib` and `wasm-lib` submodules, compiling them into WASM.
///
/// This function:
///
/// 1.  Retrieves the output directory (`OUT_DIR`) and the root project directory (`CARGO_MANIFEST_DIR`)
///     from environment variables.
/// 2.  Constructs the paths to the `mock-math-lib` and `wasm-lib` submodules.
/// 3.  Builds each submodule using `cargo build --target wasm32-unknown-unknown --release`.
/// 4.  Panics if either submodule fails to build.
/// 5.  Optionally copies the generated WASM files (`mock_math_lib.wasm` and `wasm_lib.wasm`) to a specified output directory
///     within the parent project's `target` directory (`target/wasm`).
/// 6. Prints error messages to standard output if copying fails.
/// 7.  Instructs Cargo to rerun the build script if any files in the `src` directories of the submodules change,
///     ensuring that rebuilds occur when necessary.
///
/// **Environment Variables:**
///
/// * `OUT_DIR`: The directory where Cargo places build artifacts.
/// * `CARGO_MANIFEST_DIR`: The directory containing the `Cargo.toml` of the parent project.
///
/// **File System Operations:**
///
/// * Creates the `target/wasm` directory if it does not exist.
/// * Copies the generated WASM files to the `target/wasm` directory.
///
/// **Error Handling:**
///
/// * Panics if the `cargo build` command fails for either submodule.
/// * Prints error messages to standard output if the WASM files cannot be copied.
///
/// **Rebuild Triggers:**
///
/// * Cargo will rerun this script if any files in the `mock-math-lib/src` or `wasm-lib/src` directories change.
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

    let mock_math_lib_wasm =
        mock_math_lib.join("target/wasm32-unknown-unknown/release/mock_math_lib.wasm");
    let wasm_lib_wasm = wasm_lib.join("target/wasm32-unknown-unknown/release/wasm_lib.wasm");

    let copy_mock_math_lib =
        std::fs::copy(&mock_math_lib_wasm, wasm_out_dir.join("mock_math_lib.wasm"));
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
