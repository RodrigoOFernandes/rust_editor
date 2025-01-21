# Terminal Text Editor

A **simple and lightweight terminal-based text editor** written in Rust with the help of termion crate. This application allows you to read, edit, and manage text files directly from the terminal, offering a raw terminal input mode for an enhanced editing experience.

---


## ✨ Features

- 📄 Open and display the contents of a text file.
- 📝 Create a file if it doesn't exist.
- 💾 Edit and save file contents using a terminal user interface (TUI).



## ⚙️ Prerequisites

1. Install Rust: [Get Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository to your local machine:
```
   $ git clone <repository_url>
   $ cd text_editor
```

## 🚀 Running the Program

To start the text editor, use the following command:
```
  $ cargo run <file_name>
```
The program will display the current file contents (if any) and allow you to edit it directly.

## ⌨️ Key Bindings

| Key Combination       | Action                          |
|------------------------|---------------------------------|
| `ESC`                 | Save and exit the editor       |
| `CTRL-C`              | Exit without saving      |
| `Alt + Left Arrow`    | Save the file without exiting  |


## 📜 License

This project is licensed under the MIT License.

