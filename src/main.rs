use clap::{arg, command, Arg, Command};

pub mod createproject;

fn main() {
    let matches_result = command!()
    .arg(arg!(--createproject <PROJECT_NAME>).required(true))
    .get_matches();

   let project_name = matches_result.get_one::<String>("createproject");

    match project_name {
       Some(x) =>{
            createproject::create_project::create_npm_project(x);
       }
       None =>{
        panic!("Project name is required!")
       }
    }

}
