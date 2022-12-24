use clap::Parser;

mod commands;
pub mod models;

pub fn deploy_project(project_name: &str) -> models::Params {
    let params: models::Params = Parser::parse();
    commands::cargo_compile(project_name, &params.release);
    params
}
