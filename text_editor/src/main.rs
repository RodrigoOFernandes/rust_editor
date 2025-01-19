use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

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

fn write_to_file(file_name: &String, contents: &String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) 
        .open(file_name)?;

    file.write_all(contents.as_bytes())?;
    Ok(())
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
        Ok(contents) => {
            println!("Current file contents:\n{}", contents);
            println!("\nEnter new contents (Ctrl+D to finish):");

            let mut new_contents = String::new();
            io::stdin().read_to_string(&mut new_contents).expect("Failed to read input");

            if let Err(e) = write_to_file(&file_name, &new_contents) {
                eprintln!("\nError writing to file: {}", e);
            } else {
                println!("\nFile updated successfully!");
            }
        }
        Err(e) => eprintln!("\nError reading file: {}", e),
    }
}
