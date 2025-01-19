use std::env;
use std::fs:File;


fn create_file(s: &String) -> File
{
    let file = File::create(s);
    file;
}

fn open_file(s: &String) -> File
{
    let file = File::open(s);
    file;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
}

