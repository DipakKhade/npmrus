
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

                let package_json = std::fs::read_to_string("package.json").expect("Failed to read package json");
                let updated_package_json = package_json.replace(r#" "test": "echo \"Error: no test specified\" && exit 1""#, r#" "dev":"tsc -b && node dist/index.js" "#);
                std::fs::write("package.json", updated_package_json).expect("failed to update package.json");

                let _ = std::fs::create_dir("src");
                let _ = std::env::set_current_dir("src");
                let _ = std::fs::write("index.ts", r#"
import express from 'express';

const PORT = process.env.PORT || 8080;

const app = express();

app.listen(PORT, ()=> console.log(`Server is listning on port ${PORT}`));
                "#).unwrap();

                let output = std::process::Command::new("npm")
                .arg("install")
                .arg("express")
                .arg("@types/express")
                .output()
                .expect("failed to execute npm install");
        
            if output.status.success() {
                println!("npm install succeeded");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("npm install failed:");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }

          }).join().unwrap();
        };

        create_npm_project();
        
}