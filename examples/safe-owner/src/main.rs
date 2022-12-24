use cli::deploy_project;

fn main() {
    let project_name = env!("CARGO_PKG_NAME");
    deploy_project(project_name);
}
