mod terminal;
mod file;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        return Ok(());
    }

    let file_name = &args[1];

    let contents = file::read_file_contents(file_name)?;
    println!("Current file contents:\n{}", contents);

    println!("\nStart typing (press ESC to save and exit):");
    let input_str = terminal::handle_terminal_input(contents.clone(), file_name)?;

    if let Err(e) = file::write_to_file(file_name, &input_str) {
        eprintln!("\nError writing to file: {}", e);
    } else {
        println!("\nFile updated successfully!");
    }

    Ok(())
}