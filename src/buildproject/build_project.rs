
pub fn build_dist() {
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