use std::io::{BufRead, Write};
use world::World;

pub struct Befunge<'w, 'io> {
    world: &'w mut World,
    stack: Vec<u8>,
    direction: Direction,
    x: usize,
    y: usize,
    mode: Mode,
    read: &'io mut BufRead,
    write: &'io mut Write,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Mode {
    Interpret,
    AsciiPush,
}

impl<'w, 'io> Befunge<'w, 'io> {
    pub fn new(world: &'w mut World, x: usize, y: usize, direction: Direction, read: &'io mut BufRead, write: &'io mut Write) -> Befunge<'w, 'io> {
        Befunge { world, stack: Vec::new(), direction, x, y, mode: Mode::Interpret, read, write }
    }
    pub fn run(&mut self) {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        loop {
            match self.mode {
                Mode::Interpret =>
                    match self.world.get(self.x, self.y) as char {
                        // Push this number on the stack
                        '0' => self.stack.push(0),
                        '1' => self.stack.push(1),
                        '2' => self.stack.push(2),
                        '3' => self.stack.push(3),
                        '4' => self.stack.push(4),
                        '5' => self.stack.push(5),
                        '6' => self.stack.push(6),
                        '7' => self.stack.push(7),
                        '8' => self.stack.push(8),
                        '9' => self.stack.push(9),
                        // Addition: Pop a and b, then push a+b
                        '+' => {
                            let a = self.stack.pop().unwrap_or(0);
                            let b = self.stack.pop().unwrap_or(0);
                            self.stack.push(a + b);
                        }
                        // Subtraction: Pop a and b, then push b-a
                        '-' => {
                            let a = self.stack.pop().unwrap_or(0);
                            let b = self.stack.pop().unwrap_or(0);
                            self.stack.push(b - a);
                        }
                        // Multiplication: Pop a and b, then push a*b
                        '*' => {
                            let a = self.stack.pop().unwrap_or(0);
                            let b = self.stack.pop().unwrap_or(0);
                            self.stack.push(a * b);
                        }
                        // Integer division: Pop a and b, then push b/a, rounded towards 0.
                        '/' => {
                            let a = self.stack.pop().unwrap_or(0);
                            let b = self.stack.pop().unwrap_or(0);
                            if a == 0 { return; } else { self.stack.push(b / a); }
                        }
                        // Modulo: Pop a and b, then push the remainder of the integer division of b/a.
                        '%' => {
                            let a = self.stack.pop().unwrap_or(0);
                            let b = self.stack.pop().unwrap_or(0);
                            if a == 0 { return; } else { self.stack.push(b % a); }
                        }
                        // Logical NOT: Pop a value. If the value is zero, push 1; otherwise, push zero.
                        '!' => {
                            let value = self.stack.pop().unwrap_or(0);
                            self.stack.push(if value == 0 { 1 } else { 0 });
                        }
                        // Greater than: Pop a and b, then push 1 if b>a, otherwise zero.
                        '`' => {
                            let a = self.stack.pop().unwrap_or(0);
                            let b = self.stack.pop().unwrap_or(0);
                            self.stack.push(if b > a { 1 } else { 0 });
                        }
                        // Start moving right
                        '>' => self.direction = Direction::Right,
                        // Start moving left
                        '<' => self.direction = Direction::Left,
                        // Start moving up
                        '^' => self.direction = Direction::Up,
                        // Start moving down
                        'v' => self.direction = Direction::Down,
                        // Start moving in a random cardinal direction
                        '?' => self.direction = if rng.gen() {
                            if rng.gen() { Direction::Up } else { Direction::Down }
                        } else {
                            if rng.gen() { Direction::Left } else { Direction::Right }
                        },
                        // Pop a value; move right if value=0, left otherwise
                        '_' => {
                            let value = self.stack.pop().unwrap_or(0);
                            self.direction = if value == 0 { Direction::Right } else { Direction::Left }
                        }
                        // Pop a value; move down if value=0, up otherwise
                        '|' => {
                            let value = self.stack.pop().unwrap_or(0);
                            self.direction = if value == 0 { Direction::Down } else { Direction::Up }
                        }
                        // Start string mode: push each character's ASCII value all the way up to the next
                        '"' => self.mode = Mode::AsciiPush,
                        // Duplicate value on top of the stack
                        ':' => {
                            let value = self.stack.pop().unwrap_or(0);
                            self.stack.push(value);
                            self.stack.push(value);
                        }
                        '\\' => {
                            let a = self.stack.pop().unwrap_or(0);
                            let b = self.stack.pop().unwrap_or(0);
                            self.stack.push(a);
                            self.stack.push(b);
                        }
                        // Pop value from the stack and discard it
                        '$' => {
                            self.stack.pop();
                        }
                        // Pop value and output as an integer followed by a space
                        '.' => {
                            let value = self.stack.pop().unwrap_or(0);
                            write!(&mut self.write, "{} ", value).unwrap();
                        }
                        // Pop value and output as ASCII character
                        ',' => {
                            let value = self.stack.pop().unwrap_or(0);
                            write!(&mut self.write, "{}", value as char).unwrap();
                        }
                        // Bridge: Skip next cell
                        '#' => self.forward(),
                        // A "put" call (a way to store a value for later use). Pop y, x, and v, then change the character at (x,y) in the program to the character with ASCII value v
                        'p' => {
                            let y = self.stack.pop().unwrap_or(0);
                            let x = self.stack.pop().unwrap_or(0);
                            let v = self.stack.pop().unwrap_or(0);
                            self.world.set(x as usize, y as usize, v); // TODO: Change position to relative
                        }
                        // A "get" call (a way to retrieve data in storage). Pop y and x, then push ASCII value of the character at that position in the program
                        'g' => {
                            let y = self.stack.pop().unwrap_or(0);
                            let x = self.stack.pop().unwrap_or(0);
                            let v = self.world.get(x as usize, y as usize);
                            self.stack.push(v); // TODO: Change position to relative
                        }
                        // Ask user for a number and push it
                        '&' => {
                            let mut line = String::new();
                            match self.read.read_line(&mut line) {
                                Ok(_) => {
                                    match line.trim().parse() {
                                        Ok(n) => self.stack.push(n),
                                        Err(e) => panic!(e),
                                    }
                                }
                                Err(e) => panic!(e),
                            }
                        }
                        // Ask user for a character and push its ASCII value
                        '~' => {
                            let mut buf: [u8; 1] = [0];
                            match self.read.read(&mut buf) {
                                Ok(_n) => self.stack.push(buf[0]),
                                Err(_e) => (),
                            }
                        }
                        '@' => return,
                        ' ' => {}
                        _ => {}
                    }
                Mode::AsciiPush => {
                    match self.world.get(self.x, self.y) as char {
                        '"' => self.mode = Mode::Interpret,
                        c => self.stack.push(c as u8),
                    }
                }
            }
            self.forward();
        }
    }
    fn forward(&mut self) {
        match self.direction {
            Direction::Up => self.y = if self.y == 0 { self.world.h - 1 } else { self.y - 1 },
            Direction::Down => self.y = if self.y + 1 == self.world.h { 0 } else { self.y + 1 },
            Direction::Left => self.x = if self.x == 0 { self.world.w - 1 } else { self.x - 1 },
            Direction::Right => self.x = if self.x + 1 == self.world.w { 0 } else { self.x + 1 },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world_program1() {
        use super::{Befunge, World, Direction};
        use std::io;

        let src = ">              v\nv  ,,,,,\"Hello\"<\n>48*,          v\nv,,,,,,\"World!\"<\n>25*,@";
        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(src);
        {
            let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
            befunge.run()
        }
        assert_eq!(String::from_utf8_lossy(&write[..]), "Hello World!\n");
    }

    #[test]
    fn hello_world_program2() {
        use super::{Befunge, World, Direction};
        use std::io;

        let src = "v @_       v\n>0\"!dlroW\"v\nv  :#     <\n>\" ,olleH\" v\n   ^       <";
        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(src);
        {
            let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
            befunge.run();
        }
        assert_eq!(String::from_utf8_lossy(&write[..]), "Hello, World!");
    }

    #[test]
    fn control_commands() {
        use super::{Befunge, World, Direction};
        use std::io;

        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("^    _| 3\n0#@07|5 >\n<@?@# _ v\n1 @     2");
        let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
        befunge.run();
        assert_eq!(befunge.stack, [1, 2, 3]);
    }

    #[test]
    fn literal_commands() {
        use super::{Befunge, World, Direction};
        use std::io;

        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(r#"01234"   "56789@"#);
        {
            let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
            befunge.run();
            assert_eq!(befunge.stack, [0, 1, 2, 3, 4, 0x20, 0x20, 0x20, 5, 6, 7, 8, 9]);
        }
        assert!(write.is_empty())
    }

    #[test]
    fn arithmetic_commands() {
        use super::{Befunge, World, Direction};
        use std::io;

        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("98+92-73*92/83%@");
        let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
        befunge.run();
        assert_eq!(befunge.stack, [17, 7, 21, 4, 2]);
    }

    #[test]
    fn logical_operation_commands() {
        use super::{Befunge, World, Direction};
        use std::io;

        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("8!0!12`21`@");
        let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
        befunge.run();
        assert_eq!(befunge.stack, [0, 1, 0, 1]);
    }

    #[test]
    fn stack_operation_commands() {
        use super::{Befunge, World, Direction};
        use std::io;

        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(r#"73:6\$@"#);
        let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
        befunge.run();
        assert_eq!(befunge.stack, [7, 3, 6]);
    }

    #[test]
    fn memory_operation_commands() {
        use super::{Befunge, World, Direction};
        use std::io;

        let read = Vec::new();
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("00g1+70p@");
        {
            let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
            befunge.run();
            assert_eq!(befunge.stack, []);
        }
        assert_eq!(world.get(7, 0), '1' as u8);
    }

    #[test]
    fn io_commands() {
        use super::{Befunge, World, Direction};
        use std::io;

        let read = Vec::from("51\n7\n".as_bytes());
        let mut buf_read = io::BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("&~.,@");
        {
            let mut befunge = Befunge::new(&mut world, 0, 0, Direction::Right, &mut buf_read, &mut write);
            befunge.run();
            assert_eq!(befunge.stack, []);
        }
        assert_eq!(String::from_utf8_lossy(&write[..]), "55 3");
    }
}
