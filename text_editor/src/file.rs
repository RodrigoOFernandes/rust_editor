use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub fn open_file(file_name: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
}

pub fn write_to_file(file_name: &String, contents: &String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;

    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn read_file_contents(file_name: &str) -> io::Result<String> {
    let mut file = open_file(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
