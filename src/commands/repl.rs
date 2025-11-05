use crate::Result;
use crate::protocol::ErrorCode;
use crate::repl::{CompileResult, Compiler};
use crate::serial::V4Serial;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Run interactive REPL session
pub fn run_repl(port: &str, no_reset: bool) -> Result<()> {
    // Open serial connection
    let mut serial = V4Serial::open_default(port)?;

    // Create compiler
    let mut compiler = Compiler::new().map_err(crate::V4Error::Compilation)?;

    // Create line editor
    let mut rl = DefaultEditor::new().map_err(|e| crate::V4Error::Repl(e.to_string()))?;

    // Print welcome message
    println!("V4 REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Connected to {}", port);
    println!("Type 'bye' or press Ctrl+D to exit");
    println!("Type '.help' for help");
    println!();

    // Reset device (unless --no-reset is specified)
    if no_reset {
        println!("Skipping VM reset (--no-reset)\n");
        println!("Warning: Compiler context is empty. Existing device words may not be callable.");
        println!("Use '.reset' to reset both VM and compiler context.\n");
    } else {
        println!("Resetting device...");
        match serial.reset(DEFAULT_TIMEOUT) {
            Ok(ErrorCode::Ok) => println!("Device ready\n"),
            Ok(err) => println!("Warning: Reset returned {}\n", err.name()),
            Err(e) => println!("Warning: Reset failed: {}\n", e),
        }
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
                if let Err(e) = execute_on_device(&mut serial, &compiled, &mut compiler) {
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
fn execute_on_device(
    serial: &mut V4Serial,
    compiled: &CompileResult,
    compiler: &mut Compiler,
) -> Result<()> {
    // Execute word definitions first
    for word in &compiled.words {
        eprintln!(
            "[DEBUG] Executing word '{}' ({} bytes): {:02x?}",
            word.name,
            word.bytecode.len(),
            word.bytecode
        );
        let response = serial.exec(&word.bytecode, DEFAULT_TIMEOUT)?;
        if response.error_code != ErrorCode::Ok {
            return Err(crate::V4Error::Device(format!(
                "Failed to register word '{}': {}",
                word.name,
                response.error_code.name()
            )));
        }

        // Register word index returned from device
        if let Some(&word_idx) = response.word_indices.first() {
            eprintln!(
                "[DEBUG] Device registered word '{}' at index {}",
                word.name, word_idx
            );
            compiler
                .register_word_index(&word.name, word_idx as i32)
                .map_err(crate::V4Error::Compilation)?;
        }
    }

    // Execute main bytecode
    if !compiled.bytecode.is_empty() {
        eprintln!(
            "[DEBUG] Executing main bytecode ({} bytes): {:02x?}",
            compiled.bytecode.len(),
            compiled.bytecode
        );
        let response = serial.exec(&compiled.bytecode, DEFAULT_TIMEOUT)?;
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
        ".stack" => cmd_stack(serial),
        ".rstack" => cmd_rstack(serial),
        ".dump" => cmd_dump(serial, &parts[1..]),
        ".see" => cmd_see(serial, &parts[1..]),
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
    println!("  .help              - Show this help");
    println!("  .ping              - Check device connection");
    println!("  .reset             - Reset VM and compiler context");
    println!("  .stack             - Show data and return stack contents");
    println!("  .rstack            - Show return stack with call trace");
    println!("  .dump [addr] [len] - Hexdump memory (default: continue from last)");
    println!("  .see <word_idx>    - Show word bytecode disassembly");
    println!("  .exit              - Exit REPL (same as 'bye')");
    println!("  bye                - Exit REPL");
    println!();
    println!("Forth language:");
    println!("  Any valid V4 Forth code");
    println!();
    println!("Control keys:");
    println!("  Ctrl+C   - Interrupt current line");
    println!("  Ctrl+D   - Exit REPL");
    println!("  ↑/↓      - Navigate command history");
}

/// Display data and return stacks
fn cmd_stack(serial: &mut V4Serial) -> Result<()> {
    let response = serial.query_stack(DEFAULT_TIMEOUT)?;
    if response.error_code != ErrorCode::Ok {
        return Err(crate::V4Error::Device(format!(
            "Query stack failed: {}",
            response.error_code.name()
        )));
    }

    let data = &response.data;
    if data.is_empty() {
        println!("No stack data received");
        return Ok(());
    }

    // Parse data stack
    let ds_depth = data[0] as usize;
    let mut pos = 1;

    println!("Data Stack (depth: {} / 256):", ds_depth);
    if ds_depth == 0 {
        println!("  <empty>");
    } else {
        for i in 0..ds_depth {
            if pos + 4 > data.len() {
                break;
            }
            let value = i32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            println!("  [{}]: 0x{:08X} ({})", i, value as u32, value);
            pos += 4;
        }
    }

    // Parse return stack
    if pos >= data.len() {
        return Ok(());
    }

    let rs_depth = data[pos] as usize;
    pos += 1;

    println!("\nReturn Stack (depth: {} / 64):", rs_depth);
    if rs_depth == 0 {
        println!("  <empty>");
    } else {
        for i in 0..rs_depth {
            if pos + 4 > data.len() {
                break;
            }
            let value = i32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            println!("  [{}]: 0x{:08X}", i, value as u32);
            pos += 4;
        }
    }

    Ok(())
}

/// Display return stack with call trace
fn cmd_rstack(serial: &mut V4Serial) -> Result<()> {
    let response = serial.query_stack(DEFAULT_TIMEOUT)?;
    if response.error_code != ErrorCode::Ok {
        return Err(crate::V4Error::Device(format!(
            "Query stack failed: {}",
            response.error_code.name()
        )));
    }

    let data = &response.data;
    if data.is_empty() {
        println!("No stack data received");
        return Ok(());
    }

    // Skip data stack
    let ds_depth = data[0] as usize;
    let mut pos = 1 + ds_depth * 4;

    if pos >= data.len() {
        println!("No return stack data available");
        return Ok(());
    }

    let rs_depth = data[pos] as usize;
    pos += 1;

    println!("Return Stack (depth: {} / 64):", rs_depth);
    if rs_depth == 0 {
        println!("  <empty>");
        return Ok(());
    }

    println!("\nCall trace (most recent first):");
    for i in 0..rs_depth {
        if pos + 4 > data.len() {
            break;
        }
        let value = i32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        println!("  [{:2}]: 0x{:08X}", i, value as u32);
        pos += 4;
    }

    Ok(())
}

/// Hexdump memory at address
fn cmd_dump(serial: &mut V4Serial, args: &[&str]) -> Result<()> {
    // TODO: Track last dump address for continuation
    let addr: u32 = if args.is_empty() {
        0  // Default to address 0
    } else {
        args[0].parse().map_err(|_| {
            crate::V4Error::Cli(format!("Invalid address: {}", args[0]))
        })?
    };

    let len: u16 = if args.len() < 2 {
        256  // Default to 256 bytes
    } else {
        args[1].parse::<u16>().map_err(|_| {
            crate::V4Error::Cli(format!("Invalid length: {}", args[1]))
        })?.min(256)
    };

    let response = serial.query_memory(addr, len, DEFAULT_TIMEOUT)?;
    if response.error_code != ErrorCode::Ok {
        return Err(crate::V4Error::Device(format!(
            "Query memory failed: {}",
            response.error_code.name()
        )));
    }

    let data = &response.data;
    println!("Memory dump at 0x{:08X} ({} bytes):\n", addr, data.len());

    // Display in 16-byte rows
    for (i, chunk) in data.chunks(16).enumerate() {
        let offset = addr + (i * 16) as u32;
        print!("{:08X}  ", offset);

        // Hex values
        for (j, byte) in chunk.iter().enumerate() {
            print!("{:02X} ", byte);
            if j == 7 {
                print!(" ");
            }
        }

        // Padding if last row is incomplete
        for j in chunk.len()..16 {
            print!("   ");
            if j == 7 {
                print!(" ");
            }
        }

        // ASCII representation
        print!(" |");
        for byte in chunk {
            let c = if *byte >= 0x20 && *byte <= 0x7E {
                *byte as char
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!("|");
    }

    Ok(())
}

/// Show word bytecode disassembly
fn cmd_see(serial: &mut V4Serial, args: &[&str]) -> Result<()> {
    if args.is_empty() {
        return Err(crate::V4Error::Cli("Usage: .see <word_index>".to_string()));
    }

    let word_idx: u16 = args[0].parse().map_err(|_| {
        crate::V4Error::Cli(format!("Invalid word index: {}", args[0]))
    })?;

    let response = serial.query_word(word_idx, DEFAULT_TIMEOUT)?;
    if response.error_code != ErrorCode::Ok {
        return Err(crate::V4Error::Device(format!(
            "Query word failed: {}",
            response.error_code.name()
        )));
    }

    let data = &response.data;
    if data.is_empty() {
        println!("No word data received");
        return Ok(());
    }

    // Parse response: [NAME_LEN][NAME...][CODE_LEN_L][CODE_LEN_H][CODE...]
    let name_len = data[0] as usize;
    let mut pos = 1;

    let name = if name_len > 0 && pos + name_len <= data.len() {
        String::from_utf8_lossy(&data[pos..pos + name_len]).to_string()
    } else {
        "<anonymous>".to_string()
    };
    pos += name_len;

    if pos + 2 > data.len() {
        println!("Incomplete word data");
        return Ok(());
    }

    let code_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;

    println!("Word: {}", name);
    println!("Index: {}", word_idx);
    println!("Bytecode length: {} bytes\n", code_len);

    if code_len == 0 || pos >= data.len() {
        println!("No bytecode");
        return Ok(());
    }

    println!("Disassembly:");
    println!("Offset  Bytes");
    println!("------  -------------------------");

    let code = &data[pos..];
    for (i, chunk) in code.chunks(16).enumerate() {
        print!("{:04X}    ", i * 16);
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }

    println!("\nNote: Use V4-front disassembler for opcode names.");

    Ok(())
}
