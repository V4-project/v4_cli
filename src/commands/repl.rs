use crate::Result;
use crate::protocol::ErrorCode;
use crate::repl::{CompileResult, Compiler};
use crate::serial::V4Serial;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Run interactive REPL session
pub fn run_repl(port: &str) -> Result<()> {
    // Open serial connection
    let mut serial = V4Serial::open_default(port)?;

    // Create compiler
    let mut compiler = Compiler::new().map_err(|e| crate::V4Error::Compilation(e))?;

    // Create line editor
    let mut rl = DefaultEditor::new().map_err(|e| crate::V4Error::Repl(e.to_string()))?;

    // Print welcome message
    println!("V4 REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Connected to {}", port);
    println!("Type 'bye' or press Ctrl+D to exit");
    println!("Type '.help' for help");
    println!();

    // Reset device
    println!("Resetting device...");
    match serial.reset(DEFAULT_TIMEOUT) {
        Ok(ErrorCode::Ok) => println!("Device ready\n"),
        Ok(err) => println!("Warning: Reset returned {}\n", err.name()),
        Err(e) => println!("Warning: Reset failed: {}\n", e),
    }

    // REPL loop
    loop {
        let readline = rl.readline("v4> ");

        match readline {
            Ok(line) => {
                let line = line.trim();

                // Skip empty lines
                if line.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(line);

                // Check for exit commands
                if line == "bye" || line == "quit" || line == ".exit" {
                    println!("Goodbye!");
                    break;
                }

                // Check for meta-commands
                if line.starts_with('.') {
                    if let Err(e) = handle_meta_command(line, &mut serial, &mut compiler) {
                        eprintln!("Error: {}", e);
                    }
                    continue;
                }

                // Compile Forth code
                let compiled = match compiler.compile(line) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                };

                // Execute on device
                if let Err(e) = execute_on_device(&mut serial, &compiled) {
                    eprintln!("Error: {}", e);
                    continue;
                }

                // Success
                println!(" ok");
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}

/// Execute compiled bytecode on device
fn execute_on_device(serial: &mut V4Serial, compiled: &CompileResult) -> Result<()> {
    // Execute word definitions first
    for word in &compiled.words {
        let err_code = serial.exec(&word.bytecode, DEFAULT_TIMEOUT)?;
        if err_code != ErrorCode::Ok {
            return Err(crate::V4Error::Device(format!(
                "Failed to register word '{}': {}",
                word.name,
                err_code.name()
            )));
        }
    }

    // Execute main bytecode
    if !compiled.bytecode.is_empty() {
        let err_code = serial.exec(&compiled.bytecode, DEFAULT_TIMEOUT)?;
        if err_code != ErrorCode::Ok {
            return Err(crate::V4Error::Device(format!(
                "Execution failed: {}",
                err_code.name()
            )));
        }
    }

    Ok(())
}

/// Handle meta-commands (.help, .ping, etc.)
fn handle_meta_command(line: &str, serial: &mut V4Serial, compiler: &mut Compiler) -> Result<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let command = parts[0];

    match command {
        ".help" => {
            print_help();
            Ok(())
        }
        ".ping" => {
            let err_code = serial.ping(DEFAULT_TIMEOUT)?;
            if err_code == ErrorCode::Ok {
                println!("Device is responsive");
            } else {
                println!("Device returned: {}", err_code.name());
            }
            Ok(())
        }
        ".reset" => {
            // Reset device VM
            let err_code = serial.reset(DEFAULT_TIMEOUT)?;
            if err_code != ErrorCode::Ok {
                return Err(crate::V4Error::Device(format!(
                    "Reset failed: {}",
                    err_code.name()
                )));
            }

            // Reset compiler context
            compiler.reset();

            println!("VM and compiler context reset");
            Ok(())
        }
        ".exit" => {
            // Handled in main loop
            Ok(())
        }
        _ => {
            println!("Unknown command: {}", command);
            println!("Type '.help' for available commands");
            Ok(())
        }
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  .help    - Show this help");
    println!("  .ping    - Check device connection");
    println!("  .reset   - Reset VM and compiler context");
    println!("  .exit    - Exit REPL (same as 'bye')");
    println!("  bye      - Exit REPL");
    println!();
    println!("Forth language:");
    println!("  Any valid V4 Forth code");
    println!();
    println!("Control keys:");
    println!("  Ctrl+C   - Interrupt current line");
    println!("  Ctrl+D   - Exit REPL");
    println!("  ↑/↓      - Navigate command history");
}
