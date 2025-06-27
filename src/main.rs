use clap::{arg, command};

pub mod project_init;
pub mod route_init;
pub mod build;

fn main() {
    let args = command!()
    .arg(arg!(--createproject <PROJECT_NAME>))
    .arg(arg!(--createroute <ROUTE_NAME>))
    .arg(arg!(--build <PROJECT_NAME>))
    .get_matches();

   let project_name = args.get_one::<String>("createproject");
    match project_name {
       Some(x) => project_init::create_project::create_npm_project(x),
       None => {}
    }

    let build_project = args.get_one::<String>("build");
    match build_project {
        Some(x) => build::build_project::build_dist(x),
        None => {}
    }

    let route_name = args.get_one::<String>("createroute");
    match route_name {
        Some(x) => route_init::create_route::route_init(),
        None => {}
    }

}
