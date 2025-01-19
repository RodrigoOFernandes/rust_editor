use std::env;
use std::io;
use std::fs::File;

fn create_file(s: &String) -> Result<File, io::Error>
{
    let file = match File::create(s){
        Ok(file) => file,
        Err(e) =>  return Err(e),
    };

    Ok(file)
}

fn open_file(s: &String) -> Result<File, io::Error>
{
    let file = match File::open(s){
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    Ok(file)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    create_file(file_name);
}

