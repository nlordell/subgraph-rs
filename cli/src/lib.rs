extern crate dotenv;
extern crate reqwest;

use clap::Parser;
use commands::process_params;
use dotenv::dotenv;

mod commands;
pub mod models;

pub fn deploy_project(project_name: &str) -> models::Params {
    dotenv().ok();

    let params: models::Params = Parser::parse();
    process_params(&params).expect("Bad input parameters");
    commands::cargo_compile(project_name, &params.release);
    params
}
