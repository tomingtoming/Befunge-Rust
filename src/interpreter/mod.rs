mod debug;
mod operations;
mod stack;

use crate::world::World;
pub use debug::DebugOutput;
pub use stack::Stack;
use std::error::Error;
use std::io::{BufRead, Write};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub(crate) enum Mode {
    Interpret,
    AsciiPush,
}

pub struct Interpreter<'w, 'io> {
    world: &'w mut World,
    stack: Stack,
    direction: Direction,
    x: usize,
    y: usize,
    mode: Mode,
    read: &'io mut dyn BufRead,
    write: &'io mut dyn Write,
    debug: DebugOutput,
}

impl<'w, 'io> Interpreter<'w, 'io> {
    pub fn new(
        world: &'w mut World,
        x: usize,
        y: usize,
        direction: Direction,
        read: &'io mut dyn BufRead,
        write: &'io mut dyn Write,
        debug_enabled: bool,
    ) -> Self {
        Self {
            world,
            stack: Stack::new(),
            direction,
            x,
            y,
            mode: Mode::Interpret,
            read,
            write,
            debug: DebugOutput::new(debug_enabled),
        }
    }

    // Accessor methods for debug functionality
    #[allow(dead_code)]
    pub(crate) fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub(crate) fn get_current_instruction(&self) -> char {
        self.world.get(self.x, self.y) as char
    }

    pub(crate) fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub(crate) fn get_stack(&self) -> &Stack {
        &self.stack
    }

    pub(crate) fn get_mode(&self) -> &Mode {
        &self.mode
    }

    #[allow(dead_code)]
    pub(crate) fn print_grid(&self, write: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        for y in 0..self.world.height() {
            for x in 0..self.world.width() {
                if x == self.x && y == self.y {
                    write!(write, "[{}]", self.world.get(x, y) as char)?;
                } else {
                    write!(write, " {} ", self.world.get(x, y) as char)?;
                }
            }
            writeln!(write)?;
        }
        Ok(())
    }

    // プライベートヘルパーメソッドを追加して状態をデバッグ出力
    fn write_debug_info(&mut self) -> Result<(), Box<dyn Error>> {
        let x = self.x;
        let y = self.y;
        let current_instruction = self.get_current_instruction();
        let direction = format!("{:?}", self.get_direction());
        let stack = self.get_stack().to_string();
        let mode = format!("{:?}", self.get_mode());

        writeln!(self.write, "\n=== Step Debug Info ===")?;
        writeln!(self.write, "Position: ({x}, {y})")?;
        writeln!(self.write, "Current instruction: {current_instruction}")?;
        writeln!(self.write, "Direction: {direction}")?;
        writeln!(self.write, "Stack: {stack}")?;
        writeln!(self.write, "Mode: {mode}")?;

        // グリッドの描画
        let curr_x = self.x;
        let curr_y = self.y;
        for y in 0..self.world.height() {
            for x in 0..self.world.width() {
                let ch = self.world.get(x, y) as char;
                if x == curr_x && y == curr_y {
                    write!(self.write, "[{ch}]")?;
                } else {
                    write!(self.write, " {ch} ")?;
                }
            }
            writeln!(self.write)?;
        }
        writeln!(self.write, "==================\n")?;
        Ok(())
    }

    fn debug_step(&mut self) -> Result<(), Box<dyn Error>> {
        if self.debug.is_enabled() {
            self.write_debug_info()?;

            // Pause execution in debug mode
            let mut input = String::new();
            self.read.read_line(&mut input)?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        use operations::execute_instruction;

        loop {
            self.debug_step()?;

            let instruction = self.world.get(self.x, self.y) as char;
            match self.mode {
                Mode::Interpret => {
                    if let Some(result) = execute_instruction(self, instruction)? {
                        return Ok(result);
                    }
                }
                Mode::AsciiPush => match instruction {
                    '"' => self.mode = Mode::Interpret,
                    c => self.stack.push(c as i64),
                },
            }
            self.forward();
        }
    }

    fn forward(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y = if self.y == 0 {
                    self.world.height() - 1
                } else {
                    self.y - 1
                }
            }
            Direction::Down => {
                self.y = if self.y + 1 == self.world.height() {
                    0
                } else {
                    self.y + 1
                }
            }
            Direction::Left => {
                self.x = if self.x == 0 {
                    self.world.width() - 1
                } else {
                    self.x - 1
                }
            }
            Direction::Right => {
                self.x = if self.x + 1 == self.world.width() {
                    0
                } else {
                    self.x + 1
                }
            }
        }
    }
}
