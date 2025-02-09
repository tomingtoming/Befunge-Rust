use crate::world::World;
use std::error::Error;
use std::io::{BufRead, Write};

pub struct Befunge<'w, 'io> {
    world: &'w mut World,
    stack: Vec<i64>,
    direction: Direction,
    x: usize,
    y: usize,
    mode: Mode,
    read: &'io mut dyn BufRead,
    write: &'io mut dyn Write,
    debug_mode: bool,  // 追加: デバッグモードフラグ
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]  // Mode enumにDebug traitを実装
enum Mode {
    Interpret,
    AsciiPush,
}

impl<'w, 'io> Befunge<'w, 'io> {
    pub fn new(
        world: &'w mut World,
        x: usize,
        y: usize,
        direction: Direction,
        read: &'io mut dyn BufRead,
        write: &'io mut dyn Write,
        debug_mode: bool,  // 追加: デバッグモードパラメータ
    ) -> Befunge<'w, 'io> {
        Befunge {
            world,
            stack: Vec::new(),
            direction,
            x,
            y,
            mode: Mode::Interpret,
            read,
            write,
            debug_mode,
        }
    }

    // スタックの値を文字列として整形する関数を追加
    fn format_stack_value(value: i64) -> String {
        let hex = format!("0x{:02X}", value);
        if (0x20..=0x7E).contains(&value) {
            format!("{} ('{}')", hex, char::from_u32(value as u32).unwrap())
        } else {
            hex
        }
    }

    // 追加: 実行状態を表示する関数
    fn print_debug_info(&mut self) -> Result<(), Box<dyn Error>> {
        if (!self.debug_mode) {
            return Ok(());
        }

        writeln!(self.write, "\n=== Step Debug Info ===")?;
        writeln!(self.write, "Position: ({}, {})", self.x, self.y)?;
        writeln!(self.write, "Current instruction: {}", self.world.get(self.x, self.y) as char)?;
        writeln!(self.write, "Direction: {:?}", self.direction)?;
        writeln!(self.write, "Stack: [{}]", 
            self.stack.iter()
                .map(|&v| Self::format_stack_value(v))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(self.write, "Mode: {:?}", self.mode)?;
        
        // プログラムの2D表示（現在位置をハイライト）
        for y in 0..self.world.height() {
            for x in 0..self.world.width() {
                if x == self.x && y == self.y {
                    write!(self.write, "[{}]", self.world.get(x, y) as char)?;
                } else {
                    write!(self.write, " {} ", self.world.get(x, y) as char)?;
                }
            }
            writeln!(self.write)?;
        }
        writeln!(self.write, "==================\n")?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        loop {
            if self.debug_mode {
                self.print_debug_info()?;
                // デバッグモード時は1ステップ実行後に一時停止
                let mut input = String::new();
                self.read.read_line(&mut input)?;
            }
            match self.mode {
                Mode::Interpret => match self.world.get(self.x, self.y) as char {
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
                        if a == 0 {
                            return Ok(());
                        } else {
                            self.stack.push(b / a);
                        }
                    }
                    // Modulo: Pop a and b, then push the remainder of the integer division of b/a.
                    '%' => {
                        let a = self.stack.pop().unwrap_or(0);
                        let b = self.stack.pop().unwrap_or(0);
                        if a == 0 {
                            return Ok(());
                        } else {
                            self.stack.push(b % a);
                        }
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
                    '?' => {
                        self.direction = if rng.gen() {
                            if rng.gen() {
                                Direction::Up
                            } else {
                                Direction::Down
                            }
                        } else if rng.gen() {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    }
                    // Pop a value; move right if value=0, left otherwise
                    '_' => {
                        let value = self.stack.pop().unwrap_or(0);
                        self.direction = if value == 0 {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    }
                    // Pop a value; move down if value=0, up otherwise
                    '|' => {
                        let value = self.stack.pop().unwrap_or(0);
                        self.direction = if value == 0 {
                            Direction::Down
                        } else {
                            Direction::Up
                        }
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
                        write!(&mut self.write, "{} ", value)?;
                    }
                    // Pop value and output as ASCII character
                    ',' => {
                        let value = self.stack.pop().unwrap_or(0);
                        write!(&mut self.write, "{}", char::from(value as u8))?;
                    }
                    // Bridge: Skip next cell
                    '#' => self.forward(),
                    // A "put" call (a way to store a value for later use). Pop y, x, and v, then change the character at (x,y) in the program to the character with ASCII value v
                    'p' => {
                        let y = self.stack.pop().unwrap_or(0);
                        let x = self.stack.pop().unwrap_or(0);
                        let v = self.stack.pop().unwrap_or(0);
                        self.world.set(x as usize, y as usize, v as u8); // TODO: Change position to relative
                    }
                    // A "get" call (a way to retrieve data in storage). Pop y and x, then push ASCII value of the character at that position in the program
                    'g' => {
                        let y = self.stack.pop().unwrap_or(0);
                        let x = self.stack.pop().unwrap_or(0);
                        let v = self.world.get(x as usize, y as usize);
                        self.stack.push(v as i64); // TODO: Change position to relative
                    }
                    // Ask user for a number and push it
                    '&' => {
                        let mut line = String::new();
                        self.read.read_line(&mut line)?;
                        let n = line.trim().parse()?;
                        self.stack.push(n);
                    }
                    // Ask user for a character and push its ASCII value
                    '~' => {
                        let mut buf: [u8; 1] = [0];
                        if let Ok(n) = self.read.read(&mut buf) {
                            if n > 0 {
                                self.stack.push(i64::from(buf[0]));
                            }
                        }
                    }
                    '@' => return Ok(()),
                    ' ' => {}
                    _ => {}
                },
                Mode::AsciiPush => match self.world.get(self.x, self.y) as char {
                    '"' => self.mode = Mode::Interpret,
                    c => self.stack.push(i64::from(c as u8)),
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

#[cfg(test)]
mod tests {

    use super::{Befunge, Direction, World};
    use std::error::Error;
    use std::io::BufReader;

    #[test]
    fn hello_world_program1() -> Result<(), Box<dyn Error>> {
        let src =
            ">              v\nv  ,,,,,\"Hello\"<\n>48*,          v\nv,,,,,,\"World!\"<\n>25*,@";
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(src);
        {
            let mut befunge = Befunge::new(
                &mut world,
                0,
                0,
                Direction::Right,
                &mut buf_read,
                &mut write,
                false,
            );
            befunge.run()?;
        }
        assert_eq!(String::from_utf8_lossy(&write[..]), "Hello World!\n");
        Ok(())
    }

    #[test]
    fn hello_world_program2() -> Result<(), Box<dyn Error>> {
        let src = "v @_       v\n>0\"!dlroW\"v\nv  :#     <\n>\" ,olleH\" v\n   ^       <";
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(src);
        {
            let mut befunge = Befunge::new(
                &mut world,
                0,
                0,
                Direction::Right,
                &mut buf_read,
                &mut write,
                false,
            );
            befunge.run()?;
        }
        assert_eq!(String::from_utf8_lossy(&write[..]), "Hello, World!");
        Ok(())
    }

    #[test]
    fn factorial_of_5() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("5 100p:v\nv *g00:_00g.@\n>00p1-:^");
        {
            let mut befunge = Befunge::new(
                &mut world,
                0,
                0,
                Direction::Right,
                &mut buf_read,
                &mut write,
                false,
            );
            befunge.run()?;
            assert_eq!(befunge.stack, [0]);
        }
        assert_eq!(String::from_utf8_lossy(&write[..]), "120 ");
        Ok(())
    }

    #[test]
    fn control_commands() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("^    _| 3\n0#@07|5 >\n<@?@# _ v\n1 @     2");
        let mut befunge = Befunge::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        befunge.run()?;
        assert_eq!(befunge.stack, [1, 2, 3]);
        Ok(())
    }

    #[test]
    fn literal_commands() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(r#"01234"   "56789@"#);
        {
            let mut befunge = Befunge::new(
                &mut world,
                0,
                0,
                Direction::Right,
                &mut buf_read,
                &mut write,
                false,
            );
            befunge.run()?;
            assert_eq!(
                befunge.stack,
                [0, 1, 2, 3, 4, 0x20, 0x20, 0x20, 5, 6, 7, 8, 9]
            );
        }
        assert!(write.is_empty());
        Ok(())
    }

    #[test]
    fn arithmetic_commands() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("09-9*9*98+92-73*92/83%@");
        let mut befunge = Befunge::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        befunge.run()?;
        assert_eq!(befunge.stack, [-729, 17, 7, 21, 4, 2]);
        Ok(())
    }

    #[test]
    fn logical_operation_commands() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("8!0!12`21`@");
        let mut befunge = Befunge::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        befunge.run()?;
        assert_eq!(befunge.stack, [0, 1, 0, 1]);
        Ok(())
    }

    #[test]
    fn stack_operation_commands() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string(r#"73:6\$@"#);
        let mut befunge = Befunge::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        befunge.run()?;
        assert_eq!(befunge.stack, [7, 3, 6]);
        Ok(())
    }

    #[test]
    fn memory_operation_commands() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("00g1+70p@");
        let mut befunge = Befunge::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        befunge.run()?;
        assert_eq!(befunge.stack, []);
        assert_eq!(world.get(7, 0), '1' as u8);
        Ok(())
    }

    #[test]
    fn io_commands() -> Result<(), Box<dyn Error>> {
        let read = Vec::from("-205\n7\n".as_bytes());
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("&~.,@");
        let mut befunge = Befunge::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        befunge.run()?;
        assert_eq!(befunge.stack, []);
        assert_eq!(String::from_utf8_lossy(&write[..]), "55 3");
        Ok(())
    }

    #[test]
    fn debug_mode_output() -> Result<(), Box<dyn Error>> {
        let read = Vec::from("\n\n".as_bytes());  // mutを削除
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("12+@");
        {
            let mut befunge = Befunge::new(
                &mut world,
                0,
                0,
                Direction::Right,
                &mut buf_read,
                &mut write,
                true,  // デバッグモードを有効化
            );
            befunge.run()?;
        }  // befungeのスコープを制限

        let output = String::from_utf8_lossy(&write);
        // デバッグ情報が含まれていることを確認
        assert!(output.contains("=== Step Debug Info ==="));
        assert!(output.contains("Position: (0, 0)"));
        assert!(output.contains("Current instruction: 1"));
        assert!(output.contains("Direction: Right"));
        assert!(output.contains("Stack:"));
        assert!(output.contains("Mode: Interpret"));

        Ok(())
    }

    #[test]
    fn debug_mode_disabled() -> Result<(), Box<dyn Error>> {
        let read = Vec::new();
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("12+@");
        {
            let mut befunge = Befunge::new(
                &mut world,
                0,
                0,
                Direction::Right,
                &mut buf_read,
                &mut write,
                false,  // デバッグモードを無効化
            );
            befunge.run()?;
        }  // befungeのスコープを制限
        
        let output = String::from_utf8_lossy(&write);
        // デバッグ情報が含まれていないことを確認
        assert!(!output.contains("=== Step Debug Info ==="));
        assert!(!output.contains("Position:"));

        Ok(())
    }

    #[test]
    fn debug_mode_output_ascii() -> Result<(), Box<dyn Error>> {
        let read = Vec::from("\n\n".as_bytes());
        let mut buf_read = BufReader::new(&read[..]);
        let mut write = Vec::new();
        let mut world = World::from_source_string("65@");  // 'A' のASCII値
        {
            let mut befunge = Befunge::new(
                &mut world,
                0,
                0,
                Direction::Right,
                &mut buf_read,
                &mut write,
                true,
            );
            befunge.run()?;
        }

        let output = String::from_utf8_lossy(&write);
        // デバッグ情報が含まれていることを確認
        assert!(output.contains("=== Step Debug Info ==="));
        assert!(output.contains("Position: (0, 0)"));
        assert!(output.contains("Current instruction: 6"));
        assert!(output.contains("Direction: Right"));
        // 16進数とASCII文字の表示を確認
        assert!(output.contains("Stack: [0x41 ('A')]"));
        assert!(output.contains("Mode: Interpret"));

        Ok(())
    }
}
