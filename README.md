Terminal Text Editor



A simple Rust-based terminal application for editing and managing text files. The program allows you to read, edit, and save the contents of a specified file directly from the terminal, with a raw terminal input mode for an enhanced editing experience.



Features:



Open and display the contents of a text file.

Creates file if it doesn't exist.

Edit and save the file contents using a TUI.



Prerequisites:



Install Rust.

Clone this repository to your local machine.

$ git clone ..
$ cd text_editor



Running the Program:



$ cargo run <file_name>

Replace <file_name> with the path to the file you want to edit. For example:

$ cargo run example.txt

The program will display the current file contents (if any) and allow you to edit it directly.



Key Bindings:



ESC: Save and exit the editor.

Backspace: Delete the last character.

Enter: Insert a newline.

Alt + Left Arrow: Save the file without exiting.

Project Structure:



src/main.rs: Entry point for the application. Coordinates terminal and file operations.

src/terminal.rs: Contains terminal-related functions, such as setting raw terminal mode and handling user input.

src/file.rs: Handles file-related operations like opening, reading, and writing files.



License:



This project is licensed under the MIT License.



Acknowledgments:



This project uses the Termion crate for raw terminal control and input handling.




Contributions are welcome! If you find a bug or have a feature request, feel free to open an issue or submit a pull request.
