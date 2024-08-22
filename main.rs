impl ShExCraft {
    fn compile_to_exe(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Parse the shell script into an AST
        let parser = ShellParser::new(self.script_path.clone());
        let ast = parser.parse()?;

        // Compile the AST into an executable
        let output_file = format!("{}.exe", self.script_path.replace(".sh", ""));
        let compiler = ShellCompiler::new(ast, output_file.clone());
        compiler.compile()?;

        println!("Successfully compiled to: {}", output_file);

        Ok(())
    }
}

fn main() {
    let matches = Command::new("ShExCraft")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Shell to EXE Compiler")
        .arg(
            Arg::new("command")
                .help("Command to execute (compile, run, or compile_to_exe)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("script")
                .help("Path to the shell script")
                .required(true)
                .index(2),
        )
        .get_matches();

    let command = matches.value_of("command").unwrap();
    let script_path = matches.value_of("script").unwrap().to_string();

    let shexcraft = ShExCraft::new(script_path);

    match command {
        "compile" => {
            match shexcraft.compile() {
                Ok(bytecode) => {
                    println!("Compiled bytecode:");
                    for instruction in bytecode {
                        println!("{}", instruction);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        "run" => {
            if let Err(e) = shexcraft.run() {
                eprintln!("Error: {}", e);
            }
        }
        "compile_to_exe" => {
            if let Err(e) = shexcraft.compile_to_exe() {
                eprintln!("Error: {}", e);
            }
        }
        _ => {
            eprintln!("Invalid command");
        }
    }
}
