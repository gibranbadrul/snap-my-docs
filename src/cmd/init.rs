pub fn run(config_file: Option<Option<String>>) {
    let config_path = match config_file {
        Some(Some(path)) => path,   // User provided a path
        Some(None) => "docs.json".to_string(),  // User ran `--init` without a value
        None => unreachable!(),  // This should never happen
    };

    println!("Writing default configuration to {}", config_path);
}
