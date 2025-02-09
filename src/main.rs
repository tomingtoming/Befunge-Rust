extern crate rand;

use interpreter::{Interpreter, Direction};
use std::env;
use std::fs;
use std::io::{self, BufReader};
use std::process;
use world::World;

mod interpreter;
mod world;

// Main entry point for the Befunge interpreter
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <befunge-program-file> [-d|--debug]", args[0]);
        process::exit(1);
    }

    // Parse debug mode flag from command line arguments
    let debug_mode = args.len() == 3 && (args[2] == "-d" || args[2] == "--debug");

    // Read the Befunge program from file
    let src = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", args[1], err);
            process::exit(1);
        }
    };

    let mut world = World::from_source_string(&src);
    let stdin = io::stdin();
    let mut stdin_lock = BufReader::new(stdin.lock());
    let mut stdout = io::stdout();

    // Initialize and run the Befunge interpreter
    let mut interpreter = Interpreter::new(
        &mut world,
        0,
        0,
        Direction::Right,
        &mut stdin_lock,
        &mut stdout,
        debug_mode,
    );

    if let Err(err) = interpreter.run() {
        eprintln!("Error executing Befunge program: {}", err);
        process::exit(1);
    }
}
