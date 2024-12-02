use std::fs;

pub fn read_file(filepath: &str) -> Vec<String> {
    let input_filepath = match std::env::current_dir() {
        Ok(cwd_filepath) => cwd_filepath.join(filepath),
        Err(_) => panic!("Cannot find current directory"),
    };
    println!("Input filepath: {}", input_filepath.display());
    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    file_content.split("\n").map(|s| s.to_string()).collect()
}
