use std::env;
use std::process::Command;

fn main() {
    // Optimize for release
    println!("cargo:rerun-if-changed=build.rs");

    // Ensure optimizations are used during the build
    if let Ok(target) = env::var("TARGET") {
        if target.contains("x86_64") {
            Command::new("cargo")
                .arg("build")
                .arg("--release")
                .status()
                .expect("Failed to invoke cargo build");
        }
    }
}