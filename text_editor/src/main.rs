use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use termion::{cursor, color, style, input::TermRead, raw::{IntoRawMode, RawTerminal}};
use std::io::{stdin, stdout};

fn set_terminal_mode() -> io::Result<RawTerminal<std::io::Stdout>> {
    let stdout = stdout().into_raw_mode()?;
    Ok(stdout)
}

fn reset_terminal_mode(mut stdout: RawTerminal<std::io::Stdout>) -> io::Result<()> {
    stdout.suspend_raw_mode()?;
    stdout.flush()
}

fn open_file(file_name: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
}

fn write_to_file(file_name: &String, contents: &String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) 
        .open(file_name)?;

    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        return Ok(());
    }

    let file_name = &args[1];

    let mut file = open_file(file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Current file contents:\n{}", contents);
    println!("\nStart typing (press ESC to save and exit):");

    let mut input_str = contents.clone();

    let mut stdout = set_terminal_mode()?;
    print!("{}", cursor::Hide); 

    for ch in stdin().keys() {
        let ch = ch?;

        match ch {
            termion::event::Key::Esc => break, 
            termion::event::Key::Backspace => {
                input_str.pop();
            }
            termion::event::Key::Char('\n') => {
                input_str.push('\n');
            }
            termion::event::Key::Char(c) => {
                input_str.push(c);
            }
            _ => {}
        }

        print!("{}{}", cursor::Goto(1, 1), termion::clear::All); 
        println!("Start typing (press ESC to save and exit):");

        for (i, line) in input_str.lines().enumerate() {
            print!("{}{}{}{}{}", style::Invert, color::Fg(color::LightWhite), cursor::Goto(0, 3 + i as u16), line, style::Reset);
        }

        stdout.flush()?;
    }

    print!("{}", cursor::Show);
    reset_terminal_mode(stdout)?;

    if let Err(e) = write_to_file(file_name, &input_str) {
        eprintln!("\nError writing to file: {}", e);
    } else {
        println!("\nFile updated successfully!");
    }

    Ok(())
}
