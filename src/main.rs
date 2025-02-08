extern crate rand;

use befunge::{Befunge, Direction};
use std::env;
use std::fs;
use std::io::{self, BufReader};
use std::process;
use world::World;

mod befunge;
mod world;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <befunge-program-file>", args[0]);
        process::exit(1);
    }

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
    
    let mut befunge = Befunge::new(
        &mut world,
        0,
        0,
        Direction::Right,
        &mut stdin_lock,
        &mut stdout,
    );

    if let Err(err) = befunge.run() {
        eprintln!("Error executing Befunge program: {}", err);
        process::exit(1);
    }
}
