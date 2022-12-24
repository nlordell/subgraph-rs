use std::process::Command;
use std::io::{self, Write};

pub fn cargo_compile(project_name: &str, release: &bool) {
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("-p")
        .arg(project_name);

    if *release {
        command.arg("--release");
    }

    command.arg("--target")
        .arg("wasm32-unknown-unknown");

    let output = command.output().expect("Build failure");
    
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    
    assert!(output.status.success());
}