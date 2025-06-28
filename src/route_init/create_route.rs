
pub fn route_init(route_name:&String) {
    // let _ = std::env::set_current_dir("asd");
    let index_ts_path = "src/index.ts".to_string();
    let index_ts_file = std::fs::read_to_string(&index_ts_path).expect("index.ts not found");

    let import_line = format!("import {{ {}Router }} from './routes/{}';", route_name, route_name);
    let use_line = format!("app.use(\"/{}\", {}Router);", route_name, route_name);

    let mut updated_content = String::new();

    let mut inserted_import = false;

    for line in index_ts_file.lines() {
        updated_content.push_str(line);
        updated_content.push('\n');
        if line.starts_with("import") && !inserted_import {
            inserted_import = true;
            updated_content.push_str(&import_line);
            updated_content.push('\n');
        }
    }

    let final_content = updated_content
        .replace(
            "app.listen(PORT, ()=> console.log(`Server is up...`));",
            &format!(
                "{}\n\napp.listen(PORT, ()=> console.log(`Server is up...`));",
                use_line
            )
        );

        std::fs::write(index_ts_path, final_content).expect("Unable to write to index.ts");

}