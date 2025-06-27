use clap::{arg, command};

pub mod init;
pub mod build;

fn main() {
    let matches_result = command!()
    .arg(arg!(--createproject <PROJECT_NAME>).required(true))
    .arg(arg!(--build <BUILD_PROJECT>))
    .get_matches();

   let project_name = matches_result.get_one::<String>("createproject");
    match project_name {
       Some(x) => init::create_project::create_npm_project(x),
       None => panic!("Project name is required!")
    }

    let build_project = matches_result.get_one::<String>("build");

    match build_project {
        Some(x) => build::build_project::build_dist(),
        None => {}
    }

}
