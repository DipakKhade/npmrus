
pub fn build_dist(project_name:&String) {
    let _ = std::env::set_current_dir(project_name);
    if !std::path::Path::new("package.json").exists() { 
        panic!("package.json Not Found!")
    };

    let cmd = std::process::Command::new("npm")
                                        .arg("run")
                                        .arg("build")
                                        .output()
                                        .expect("failed to build the project");

    if cmd.status.success() {
        println!("Build Success!")
    } else {
        println!("Build Failed!")
    }
}