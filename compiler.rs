use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::error::Error;

pub struct ShellCompiler {
    ast: Vec<Vec<String>>,
    output_file: String,
}

impl ShellCompiler {
    pub fn new(ast: Vec<Vec<String>>, output_file: String) -> Self {
        ShellCompiler { ast, output_file }
    }

    pub fn compile(&self) -> Result<(), Box<dyn Error>> {
        let mut bytecode = Vec::new();

        for command in &self.ast {
            let instruction = self.compile_command(command);
            bytecode.push(instruction);
        }

        // Write the compiled bytecode to a temporary script file
        let script_filename = "temp_script.sh";
        let mut script_file = File::create(script_filename)?;
        for instruction in bytecode {
            writeln!(script_file, "{}", instruction)?;
        }

        // Compile the script into a .exe using a cross-compilation tool (e.g., using mingw-w64)
        let output = Command::new("x86_64-w64-mingw32-gcc")
            .args(&[script_filename, "-o", &self.output_file])
            .output()?;

        if !output.status.success() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Failed to create .exe: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            )));
        }

        Ok(())
    }

    fn compile_command(&self, command: &Vec<String>) -> String {
        format!("EXEC {}", command.join(" "))
    }
}
