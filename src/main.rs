extern crate rand;

use befunge::{Befunge, Direction};
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use world::World;

mod befunge;
mod world;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut src = String::new();
    let _ = fs::File::open(&args[1])
        .expect("File not found")
        .read_to_string(&mut src);
    let mut world = World::from_source_string(&mut src[..]);
    let mut buf_read = io::BufReader::new(io::stdin());
    let mut write = io::stdout();
    let mut befunge = Befunge::new(
        &mut world,
        0,
        0,
        Direction::Right,
        &mut buf_read,
        &mut write,
    );
    befunge.run()
}
