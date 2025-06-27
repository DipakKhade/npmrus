
pub fn create_npm_project(project_dir_name: &String) {
        let is_dir_exists = std::path::Path::new(project_dir_name).is_dir();
        if is_dir_exists {
            panic!("directory already exists!");
        }
        std::fs::create_dir(project_dir_name).unwrap();
        std::env::set_current_dir(project_dir_name).unwrap();
        let _ = std::env::current_dir().unwrap();

        let create_npm_project = || {
            let _handler = std::thread::spawn(move || {
                let _ = std::process::Command::new("npm").arg("init").arg("-y").output().expect("failed to create a project");

                let _ = std::process::Command::new("npx").arg("tsc").arg("--init").output().expect("failed to create a project");

                let tsconfig = std::fs::read_to_string("tsconfig.json").expect("Failed to read tsconfig.json");

                let updated = tsconfig
                    .replace(
                        r#"// "rootDir": "./","#,
                        r#""rootDir": "./src","#,
                    )
                    .replace(
                        r#"// "outDir": "./","#,
                        r#""outDir": "./dist","#,
                    );

                std::fs::write("tsconfig.json", updated).expect("Failed to write updated tsconfig.json");

                let _ = std::fs::create_dir("src");
                let _ = std::env::set_current_dir("src");
                let _ = std::fs::write("index.ts", r#"
function main() {
    console.log('Hello world')
};

main();
                "#).unwrap();

          }).join().unwrap();
        };

        create_npm_project();
        
}