use std::{fs, io};
use std::io::Read;

pub fn run(file_path: &str) {
    println!("Analyzing file: {}", file_path);

    match fs::metadata(file_path) {
        Ok(_) => match read_binary_file(file_path) {
            Ok(data) => {
                println!("Read {} bytes from file.", data.len());
            },
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        },
        Err(e) => {
            eprintln!("File does not exist or cannot be accessed: {}", e);
        }
    }
}

fn read_binary_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
