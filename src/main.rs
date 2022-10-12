use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let file_contents = fs::read_to_string(file_path)
        .expect("Unable to read the file or you don't have the necessary permissions");

    println!("File Contents: \n{}", file_contents);
}
