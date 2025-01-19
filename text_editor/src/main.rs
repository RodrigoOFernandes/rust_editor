use std::env;
//use std::io;
use std::fs::File;
use std::fs::OpenOptions;

fn create_file(s: &String) -> ()
{
    let _file = match File::create(s){
        Ok(file) => file,
        Err(e) => {
            println!("Erro a criar o ficheiro; {e}");
            return;
        },
    };
}

fn open_file(s: &String) -> ()
{
    let _file = match OpenOptions::new().append(true).open(s){
        Ok(file) => file,
        Err(e) => {
            println!("Erro a abrir o ficheiro: {e}");
            return;
        },
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    create_file(file_name);
    open_file(file_name);
}

