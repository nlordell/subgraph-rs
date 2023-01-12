use std::process::Command;
use std::io::{self, Write};
use std::collections::HashMap;

use crate::models::Params;
use std::env;

pub fn process_params(params: &Params) -> Result<(), Box<dyn std::error::Error>> { 
    let graph_slug = params.graph_slug.to_owned().unwrap_or_else(|| {
        env::var("GRAPH_SLUG")
            .expect("You can pass the option --graph-slug or set the env variable GRAPH_SLUG")
    });
    
    let graph_studio_token = params.graph_studio_token.to_owned().unwrap_or_else(|| {
        env::var("GRAPH_STUDIO_TOKEN")
            .expect("You can pass the option --graph-studio-token or set the env variable GRAPH_STUDIO_TOKEN")
    });

    Ok(())
}

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

    add_ipfs("file_name").expect("Adding to ipfs failure");
}


pub fn add_ipfs(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}