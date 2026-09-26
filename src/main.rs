use std::{env, path};

use arc::{builder::BytecodeBuilder, bytecode::{Bytecode, Opcode}, runtime::{Value, Vm}};

fn main() {
    const TEST: bool = false;

    if TEST {
        test();
    } else {
        cli();
    }
}

#[allow(dead_code)]
fn cli() {
    let mut args = env::args();
    args.next();

    let cmd = args.next();

    if let Err(err) = match cmd.as_deref() {
        Some("help") => handle_help(),
        Some("run") => handle_run(&mut args),
        Some(cmd) => {
            eprintln!("[Error] Unknown command: '{cmd}'.\n--> Use 'help' for list of commands.");
            std::process::exit(1);
        }
        None => {
            eprintln!("[Error] No command provided.\n--> Use 'help' for list of commands.");
            std::process::exit(1);
        }
    } {
        eprintln!("[Error] {err}");
        std::process::exit(1);
    }
}

#[allow(dead_code)]
fn test() {
    let mut builder = BytecodeBuilder::new();

    builder
        .ldc(Value::Int(42))
        .ldc(Value::Int(8))
        .op(Opcode::Add)
        .ldc(Value::Int(2))
        .op(Opcode::Mul)
        .op(Opcode::Dup)
        .op(Opcode::DebugPrint)
        .op(Opcode::Print)
        .op(Opcode::Halt);

    let bytecode = builder.finish();

    bytecode
        .write_to("hello.arx")
        .expect("Failed to write bytecode.");

    println!("Wrote hello.arx");

    let bytecode = Bytecode::read_from("hello.arx")
        .expect("Failed to read bytecode.");

    println!("Code: {:?}", bytecode.code);
    println!("Constants: {:?}", bytecode.constants);
}

fn handle_help() -> Result<(), String> {
    println!("List of commands:");
    println!("> arc help");
    println!("> arc run <.arx binary>");
    Ok(())
}

fn handle_run(args: &mut impl Iterator<Item = String>) -> Result<(), String> {
    let input = args
        .next()
        .ok_or("No '.arx' binary provided.")?;

    let mut path = path::PathBuf::from(input);

    if path.extension().is_none() {
        path.set_extension("arx");
    }

    if path.extension().and_then(|s| s.to_str()) != Some("arx") {
        return Err(format!(
            "Invalid file extension. The VM only executes '.arx' binaries."
        ));
    }

    if !path.is_file() {
        return Err(format!(
            "File not found '{}'.",
            path.display()
        ));
    }

    let bytecode = Bytecode::read_from(&path)
        .map_err(|err|
            format!(
                "Failed to read '{}': {err}",
                path.display()
            )
        )?;

    /*
    ? DEBUG: println!("Successfully loaded: '{}'.", path.display());
    ? DEBUG: println!("Code count: {} bytes", bytecode.code.len());
    ? DEBUG: println!("Constants count: {}", bytecode.constants.len());
    */

    let mut vm = Vm::new(bytecode);
    vm.run()
        .map_err(|err|
            err.to_string()
        )?;

    Ok(())
}