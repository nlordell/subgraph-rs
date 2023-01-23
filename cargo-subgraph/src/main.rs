extern crate dotenv;
extern crate reqwest;

use clap::Parser;
use commands::param_validation;
use dotenv::dotenv;

mod commands;
pub mod models;

fn main() {
    dotenv().ok();

    let params: models::Params = Parser::parse();
    param_validation(&params).expect("Bad input parameters");

    deploy_project(&params.example_name, &params.release)
}

pub fn deploy_project(project_name: &str, is_release: &bool) {
    commands::cargo_compile(project_name, is_release);
}
