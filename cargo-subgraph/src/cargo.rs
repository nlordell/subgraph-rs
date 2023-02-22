use std::process::Command;
use std::io::{self, Write};

pub fn compile_project_wasm(project_name: &str, release: &bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("-p")
        .arg(project_name);

    if *release {
        command.arg("--release");
    }

    // TODO: add a check before attempting compiling for this target. The resulting error is very cryptic.
    command.arg("--target")
        .arg("wasm32-unknown-unknown");

    let output = command.output().expect("Build failure");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    assert!(output.status.success());

    Ok(String::from("Path to wasm binary"))
}
