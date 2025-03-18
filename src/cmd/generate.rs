pub fn run(config: &std::path::PathBuf, output: &Option<std::path::PathBuf>) {
    println!("Generating documentation using config: {:?}", config);
    if let Some(out) = output {
        println!("Output will be written to: {:?}", out);
    }
}
