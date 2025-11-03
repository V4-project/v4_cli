use crate::Result;
use crate::protocol::ErrorCode;
use crate::repl::Compiler;
use crate::serial::V4Serial;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs;
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Execute Forth source file on device
pub fn exec(file: &str, port: &str, timeout: Duration, enter_repl: bool) -> Result<()> {
    // Read Forth source file
    let source = fs::read_to_string(file)?;

    // Open serial connection
    let mut serial = V4Serial::open_default(port)?;

    // Create compiler
    let mut compiler = Compiler::new().map_err(crate::V4Error::Compilation)?;

    println!("Compiling {}...", file);

    // Compile Forth source
    let compiled = compiler
        .compile(&source)
        .map_err(crate::V4Error::Compilation)?;

    // Send word definitions first
    if !compiled.words.is_empty() {
        println!("Compiled {} word(s)", compiled.words.len());

        for word in &compiled.words {
            println!(
                "  Sending word '{}'... ({} bytes)",
                word.name,
                word.bytecode.len()
            );

            let response = serial.exec(&word.bytecode, timeout)?;

            if response.error_code != ErrorCode::Ok {
                eprintln!("  Error: {}", response.error_code.name());
                return Err(crate::V4Error::Protocol(format!(
                    "Device returned error: {}",
                    response.error_code.name()
                )));
            }

            // Register word in compiler context
            if let Some(&word_idx) = response.word_indices.first() {
                println!("  Word '{}' registered at index {}", word.name, word_idx);
                compiler
                    .register_word_index(&word.name, word_idx as i32)
                    .map_err(crate::V4Error::Compilation)?;
            }
        }
    }

    // Execute main bytecode if present
    if !compiled.bytecode.is_empty() {
        println!(
            "Executing main bytecode... ({} bytes)",
            compiled.bytecode.len()
        );

        let response = serial.exec(&compiled.bytecode, timeout)?;

        if response.error_code != ErrorCode::Ok {
            eprintln!("Error: {}", response.error_code.name());
            return Err(crate::V4Error::Protocol(format!(
                "Execution failed: {}",
                response.error_code.name()
            )));
        }

        println!("Execution complete");
    } else if !compiled.words.is_empty() {
        println!("Word definitions complete");
    }

    // Enter REPL if requested
    if enter_repl {
        println!("\nEntering REPL...");
        println!("Type 'bye' or press Ctrl+D to exit");
        println!("Type '.help' for help\n");

        let mut rl = DefaultEditor::new().map_err(|e| crate::V4Error::Repl(e.to_string()))?;

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
                    if let Err(e) =
                        execute_on_device(&mut serial, &compiled, &mut compiler, timeout)
                    {
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
    }

    Ok(())
}

/// Execute compiled bytecode on device
fn execute_on_device(
    serial: &mut V4Serial,
    compiled: &crate::repl::CompileResult,
    compiler: &mut Compiler,
    timeout: Duration,
) -> Result<()> {
    // Execute word definitions first
    for word in &compiled.words {
        let response = serial.exec(&word.bytecode, timeout)?;
        if response.error_code != ErrorCode::Ok {
            return Err(crate::V4Error::Device(format!(
                "Failed to register word '{}': {}",
                word.name,
                response.error_code.name()
            )));
        }

        // Register word index returned from device
        if let Some(&word_idx) = response.word_indices.first() {
            compiler
                .register_word_index(&word.name, word_idx as i32)
                .map_err(crate::V4Error::Compilation)?;
        }
    }

    // Execute main bytecode
    if !compiled.bytecode.is_empty() {
        let response = serial.exec(&compiled.bytecode, timeout)?;
        if response.error_code != ErrorCode::Ok {
            return Err(crate::V4Error::Device(format!(
                "Execution failed: {}",
                response.error_code.name()
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
            println!("Available commands:");
            println!("  .help   - Show this help");
            println!("  .ping   - Ping device");
            println!("  .reset  - Reset VM and compiler context");
            println!("  .exit   - Exit REPL");
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
