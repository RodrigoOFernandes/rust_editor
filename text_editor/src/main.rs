use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read};

fn create_file(file_name: &str) {
    match File::create(file_name) {
        Ok(_) => println!("File created successfully: {}", file_name),
        Err(e) => eprintln!("Error creating the file: {e}"),
    }
}

fn open_file(file_name: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) 
        .open(file_name)
}

fn read_file(file: &mut File) -> io::Result<String> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        return;
    }

    let file_name = &args[1];

    let mut file = match open_file(file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    match read_file(&mut file) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

