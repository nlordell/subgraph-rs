extern crate dotenv;
extern crate reqwest;

use clap::Parser;
use commands::param_validation;
use dotenv::dotenv;

mod commands;
mod cargo;
mod ipfs_client;
pub mod models;

fn main() {
    dotenv().ok();

    let params: models::Params = Parser::parse();
    let graph = param_validation(&params).expect("Bad input parameters");

    let wasm_bin_path = cargo::compile_project_wasm(&params.example_name, &params.release)
        .expect("Error compiling WASM binary for graph project");
    ipfs_client::add_ipfs(&wasm_bin_path)
        .expect("Error deploying WASM binary into IPFS");
}
