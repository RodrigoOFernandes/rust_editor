use std::io::{self, Write};
use termion::{cursor, color, style, input::TermRead, raw::{IntoRawMode, RawTerminal}};
use std::io::{stdin, stdout};

pub fn set_terminal_mode() -> io::Result<RawTerminal<std::io::Stdout>> {
    let stdout = stdout().into_raw_mode()?;
    Ok(stdout)
}

pub fn reset_terminal_mode(mut stdout: RawTerminal<std::io::Stdout>) -> io::Result<()> {
    stdout.suspend_raw_mode()?;
    stdout.flush()
}

pub fn handle_terminal_input(mut input_str: String, file_name: &String) -> io::Result<String> {
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
            termion::event::Key::AltLeft => {
                if let Err(e) = crate::file::write_to_file(file_name, &input_str) {
                    eprintln!("\nError writing to file: {}", e);
                }
            }
            termion::event::Key::Char(c) => {
                input_str.push(c);
            }
            _ => {}
        }

        print!("{}{}", cursor::Goto(1, 1), termion::clear::All);
        println!("Start typing (press ESC to save and exit, CTL-C to exit without saving):");

        for (i, line) in input_str.lines().enumerate() {
            print!("{}{}{}{}{}", style::Invert, color::Fg(color::LightWhite), cursor::Goto(0, 3 + i as u16), line, style::Reset);
        }

        stdout.flush()?;
    }

    print!("{}", cursor::Show);
    reset_terminal_mode(stdout)?;
    Ok(input_str)
}