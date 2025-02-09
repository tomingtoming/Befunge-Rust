use std::error::Error;
use rand::{thread_rng, Rng};
use crate::interpreter::{Direction, Interpreter, Mode};

pub(crate) fn execute_instruction<'w, 'io>(
    interpreter: &mut Interpreter<'w, 'io>,
    instruction: char,
) -> Result<Option<()>, Box<dyn Error>> {
    match instruction {
        // Push this number on the stack
        '0'..='9' => {
            interpreter.stack.push(instruction.to_digit(10).unwrap() as i64);
            Ok(None)
        }
        // Addition: Pop a and b, then push a+b
        '+' => {
            let a = interpreter.stack.pop();
            let b = interpreter.stack.pop();
            interpreter.stack.push(a + b);
            Ok(None)
        }
        // Subtraction: Pop a and b, then push b-a
        '-' => {
            let a = interpreter.stack.pop();
            let b = interpreter.stack.pop();
            interpreter.stack.push(b - a);
            Ok(None)
        }
        // Multiplication: Pop a and b, then push a*b
        '*' => {
            let a = interpreter.stack.pop();
            let b = interpreter.stack.pop();
            interpreter.stack.push(a * b);
            Ok(None)
        }
        // Integer division: Pop a and b, then push b/a
        '/' => {
            let a = interpreter.stack.pop();
            if a == 0 {
                Ok(Some(()))
            } else {
                let b = interpreter.stack.pop();
                interpreter.stack.push(b / a);
                Ok(None)
            }
        }
        // Modulo: Pop a and b, then push b%a
        '%' => {
            let a = interpreter.stack.pop();
            if a == 0 {
                Ok(Some(()))
            } else {
                let b = interpreter.stack.pop();
                interpreter.stack.push(b % a);
                Ok(None)
            }
        }
        // Logical NOT
        '!' => {
            let value = interpreter.stack.pop();
            interpreter.stack.push(if value == 0 { 1 } else { 0 });
            Ok(None)
        }
        // Greater than
        '`' => {
            let a = interpreter.stack.pop();
            let b = interpreter.stack.pop();
            interpreter.stack.push(if b > a { 1 } else { 0 });
            Ok(None)
        }
        // Direction control
        '>' => {
            interpreter.direction = Direction::Right;
            Ok(None)
        }
        '<' => {
            interpreter.direction = Direction::Left;
            Ok(None)
        }
        '^' => {
            interpreter.direction = Direction::Up;
            Ok(None)
        }
        'v' => {
            interpreter.direction = Direction::Down;
            Ok(None)
        }
        // Random direction
        '?' => {
            let mut rng = thread_rng();
            interpreter.direction = if rng.gen() {
                if rng.gen() {
                    Direction::Up
                } else {
                    Direction::Down
                }
            } else if rng.gen() {
                Direction::Left
            } else {
                Direction::Right
            };
            Ok(None)
        }
        // Pop and branch
        '_' => {
            let value = interpreter.stack.pop();
            interpreter.direction = if value == 0 {
                Direction::Right
            } else {
                Direction::Left
            };
            Ok(None)
        }
        '|' => {
            let value = interpreter.stack.pop();
            interpreter.direction = if value == 0 {
                Direction::Down
            } else {
                Direction::Up
            };
            Ok(None)
        }
        // String mode
        '"' => {
            interpreter.mode = Mode::AsciiPush;
            Ok(None)
        }
        // Stack operations
        ':' => {
            interpreter.stack.duplicate_top();
            Ok(None)
        }
        '\\' => {
            interpreter.stack.swap_top();
            Ok(None)
        }
        '$' => {
            interpreter.stack.pop();
            Ok(None)
        }
        // Output operations
        '.' => {
            let value = interpreter.stack.pop();
            write!(interpreter.write, "{} ", value)?;
            Ok(None)
        }
        ',' => {
            let value = interpreter.stack.pop();
            write!(interpreter.write, "{}", char::from(value as u8))?;
            Ok(None)
        }
        // Bridge
        '#' => {
            interpreter.forward();
            Ok(None)
        }
        // Memory operations
        'p' => {
            let y = interpreter.stack.pop();
            let x = interpreter.stack.pop();
            let v = interpreter.stack.pop();
            interpreter.world.set(x as usize, y as usize, v as u8);
            Ok(None)
        }
        'g' => {
            let y = interpreter.stack.pop();
            let x = interpreter.stack.pop();
            let v = interpreter.world.get(x as usize, y as usize);
            interpreter.stack.push(v as i64);
            Ok(None)
        }
        // Input operations
        '&' => {
            let mut line = String::new();
            interpreter.read.read_line(&mut line)?;
            let n = line.trim().parse()?;
            interpreter.stack.push(n);
            Ok(None)
        }
        '~' => {
            let mut buf = [0u8; 1];
            if let Ok(n) = interpreter.read.read(&mut buf) {
                if n > 0 {
                    interpreter.stack.push(i64::from(buf[0]));
                }
            }
            Ok(None)
        }
        // Program termination
        '@' => Ok(Some(())),
        // Space (no operation)
        ' ' => Ok(None),
        // Unknown instruction (no operation)
        _ => Ok(None),
    }
}