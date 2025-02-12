mod befunge;
mod world;

use wasm_bindgen::prelude::*;
use crate::befunge::{Befunge, Direction};
use crate::world::World;
use std::io::{BufReader, Cursor};

#[wasm_bindgen]
pub struct WebBefunge {
    program: String,
    input: String,
    output: Vec<u8>,
}

#[wasm_bindgen]
impl WebBefunge {
    #[wasm_bindgen(constructor)]
    pub fn new(program: String) -> Self {
        WebBefunge {
            program,
            input: String::new(),
            output: Vec::new(),
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    pub fn run(&mut self) -> Result<String, JsValue> {
        // Set up input and output
        let input = Cursor::new(self.input.as_bytes());
        let mut buf_read = BufReader::new(input);
        self.output.clear();

        // Initialize the world and interpreter
        let mut world = World::from_source_string(&self.program);
        let mut befunge = Befunge::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut self.output,
        );

        // Run the program
        befunge.run().map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Convert output to string
        String::from_utf8(self.output.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}