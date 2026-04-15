use std::{io::Write, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum EnumValues {
    /// Build the DLL
    #[value(name = "build-dll", help = "Build the DLL for the C API")]
    BuildDLL,
    #[value(
        name = "generate-bindings",
        help = "Generate Rust bindings for the C API"
    )]
    GenerateBindings,
    #[value(
        name = "test-all",
        help = "Run all tests with CHOCO_SOLVER_DLL_FOLDER set"
    )]
    TestAll,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command to execute
    #[arg(value_enum)]
    command: EnumValues,
}
const HEADER_FILENAME: &str = "libchoco_capi.h";
fn main() {
    let args = Args::parse();
    match args.command {
        EnumValues::GenerateBindings => {
            println!("Generating bindings...");
            let out_dir = PathBuf::from("./choco-solver-sys/src");
            let headers_dir = PathBuf::from("./choco-solver-sys/choco-solver-capi/target")
                .canonicalize()
                .unwrap();
            let header_file = headers_dir.join(HEADER_FILENAME);
            if !header_file.exists() {
                panic!(
                    "Header file not found at: {}, forgotten to run build-dll ?",
                    header_file.display()
                );
            }
            // Tell Cargo that if the given file changes, to rerun this build script.

            // The bindgen::Builder is the main entry point
            // to bindgen, and lets you build up options for
            // the resulting bindings.
            let bindings = bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header(header_file.to_str().unwrap())
                .clang_arg("-I".to_string() + headers_dir.to_str().unwrap())
                .wrap_unsafe_ops(true)
                .dynamic_library_name("libchoco_capi")
                .opaque_type("__graal_create_isolate_params_t")
                .blocklist_type("__graal_uword")
                .derive_debug(false)
                .derive_default(false)
                .derive_copy(false)
                // Finish the builder and generate the bindings.
                .generate()
                // Unwrap the Result and panic on failure.
                .expect("Unable to generate bindings");
            let mut bindings_file = Box::new(
                std::fs::File::create(out_dir.join("bindings.rs"))
                    .expect("Couldn't create bindings file!"),
            );
            bindings_file
                .write_all(
                    r#"
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
// Auto-generated bindings for libchoco_capi.h                        
"#
                    .to_string()
                    .as_bytes(),
                )
                .expect("Couldn't write header to bindings file!");
            bindings
                .write(bindings_file)
                .expect("Couldn't write bindings!");
        }
        EnumValues::BuildDLL => {
            println!("Building DLL...");

            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            let choco_solver_capi_source_folder =
                current_dir.join("./choco-solver-sys/choco-solver-capi");
            std::env::set_current_dir(&choco_solver_capi_source_folder).expect(
                "Failed to set current working directory to choco_solver_capi_source_folder",
            );

            let mvn_command = match find_command(&["mvn", "mvn.cmd", "mvn.exe"][..]) {
                Some(cmd) => cmd,
                None => {
                    println!(
                        "Required tool `Maven` was not found in PATH. Please install it and ensure it is available from the command line."
                    );
                    panic!("Maven not found")
                }
            };
            // Ensure GraalVM is configured
            if std::env::var_os("GRAALVM_HOME").is_none() {
                panic!(
                    "Required environment variable `GRAALVM_HOME` is not set. \
Please set it before running `build-dll`."
                );
            }
            // Clean up previous builds
            execute_command(
                &mvn_command,
                &["clean", "package"],
                &choco_solver_capi_source_folder,
            );
            // Buidl native image using maven
            execute_command(
                &mvn_command,
                &["-Pnative", "package"],
                &choco_solver_capi_source_folder,
            );

            std::env::set_current_dir(&current_dir)
                .expect("Failed to set current working directory to original directory");
        }
        EnumValues::TestAll => {
            println!("Running tests...");
            let headers_dir = PathBuf::from("./choco-solver-sys/choco-solver-capi/target")
                .canonicalize()
                .unwrap();
            let mut cmd = std::process::Command::new("cargo");
            cmd.args(["test"])
                .env("CHOCO_SOLVER_DLL_FOLDER", headers_dir.to_str().unwrap());
            let status = cmd.status().expect("Failed to run cargo test");
            if !status.success() {
                panic!("cargo test failed with status: {}", status);
            }
        }
    }
}
fn find_command(candidates: &[&str]) -> Option<String> {
    candidates.iter().find_map(|cmd| {
        if std::process::Command::new(cmd)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok()
        {
            Some(cmd.to_string())
        } else {
            None
        }
    })
}

fn execute_command(cmd: &str, args: &[&str], dir: impl AsRef<std::path::Path>) {
    let mut child = std::process::Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to run Maven command (`{} {:?}`)", cmd, args));
    let status = child.wait().expect("Failed to complete command");
    if !status.success() {
        panic!("`{} {:?}` failed with status: {}", cmd, args, status);
    }
}
