# ShExCraft

ShExCraft is a tool that transforms shell scripts (.sh files) into executable files (.exe) on Windows. It ensures command availability, simplifies command-line argument handling, and provides detailed error reporting during compilation and execution.

## Features

- **Parse Shell Scripts**: Converts shell scripts into an Abstract Syntax Tree (AST).
- **Compile to Bytecode**: Transforms the AST into bytecode instructions.
- **Compile to .EXE**: Generates a `.exe` file from the shell script for execution on Windows.
- **Run Shell Scripts**: Executes shell scripts directly with detailed error handling.
- **Error Handling**: Provides precise error messages for syntax errors, missing commands, and other issues.

## Installation

### Prerequisites

- **Rust**: Ensure you have Rust installed on your system. If not, install it from [rust-lang.org](https://www.rust-lang.org/).
- **Mingw-w64** (for cross-compiling to `.exe` on Unix-like systems):
  ```bash
  sudo apt-get install mingw-w64

# Clone the Repository
Clone the ShExCraft repository to your local machine:
```bash
git clone https://github.com/YOUR-USERNAME/ShExCraft.git
cd ShExCraft
```

# Build the Project
Build the project using Cargo:
```bash
cargo build --release
```
# Usage
Compiling a Shell Script
To compile a shell script into bytecode:
```bash
./target/release/shexcraft compile your_script.sh
```

Running a Shell Script
To run a shell script with ShExCraft:
```bash
./target/release/shexcraft run your_script.sh
```

Compiling to .EXE
To compile a shell script directly into an executable .exe file:
```bash
./target/release/shexcraft compile_to_exe your_script.sh
```

The output will be your_script.exe in the current directory, which you can then run on a Windows system.

# Example
Example Shell Script (example.sh)
```bash
#!/bin/bash
echo "Welcome to ShExCraft!"
mkdir new_directory
cd new_directory
touch file1.txt file2.txt
echo "Files created successfully"
```
# Example Commands
Compile the script to bytecode:

```bash
./target/release/shexcraft compile example.sh
Run the script with error handling:
```

```bash
./target/release/shexcraft run example.sh
Compile the script to .exe:
```

```bash
./target/release/shexcraft compile_to_exe example.sh
Error Handling
ShExCraft provides detailed error messages in case of any issues during parsing, compilation, or execution. Here's an example of what might happen if there's an error:
```

```bash
Error: Interpreter error

Caused by:
    Command failed: mkdir new_directory
    error: File exists
```
License
This project is licensed under the MIT License - see the LICENSE file for details.

Contributing
Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

Contact
For questions or support, please reach out via GitHub issues or contact my email.
