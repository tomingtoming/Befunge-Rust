use std::io;

pub struct World {
    width: usize,
    height: usize,
    field: Vec<Vec<u8>>,
}

impl World {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn from_source_string(source: &str) -> io::Result<World> {
        let mut world: Vec<Vec<u8>> = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        let width = lines.iter().fold(0, |i, s| i.max(s.len()));

        if width == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "empty befunge program",
            ));
        }

        for line in lines {
            let mut belt = Vec::from(line);
            while belt.len() != width {
                belt.push(b' ');
            }
            world.push(belt)
        }

        Ok(World {
            width,
            height: world.len(),
            field: world,
        })
    }

    #[allow(dead_code)]
    pub fn from_random(width: usize, height: usize) -> World {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let mut world: Vec<Vec<u8>> = Vec::new();
        for _ in 0..height {
            let mut line = Vec::with_capacity(width);
            for _ in 0..width {
                line.push(rng.gen());
            }
            world.push(line);
        }
        World {
            width,
            height,
            field: world,
        }
    }

    #[allow(dead_code)]
    pub fn println(&self) {
        for line in &self.field {
            for &elem in line {
                if (0x20..=0x7e).contains(&elem) {
                    print!("{}", char::from(elem));
                } else {
                    print!("□");
                }
            }
            println!();
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.field[y % self.height][x % self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.field[y % self.height][x % self.width] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn hello_world_program() -> std::io::Result<()> {
        let src =
            ">              v\nv  ,,,,,\"Hello\"<\n>48*,          v\nv,,,,,,\"World!\"<\n>25*,@";
        let world = World::from_source_string(src)?;
        assert_eq!(world.get(0, 0), b'>');
        assert_eq!(world.get(15, 0), b'v');
        assert_eq!(world.get(15, 4), b' ');
        assert_eq!(world.get(0, 4), b'>');
        assert_eq!(world.get(16, 5), b'>');
        Ok(())
    }

    #[test]
    fn trailing_newline_does_not_add_an_empty_row() -> std::io::Result<()> {
        let world = World::from_source_string(">@\n")?;
        assert_eq!(world.width(), 2);
        assert_eq!(world.height(), 1);
        Ok(())
    }

    #[test]
    fn empty_program_returns_error() {
        let err = World::from_source_string("")
            .err()
            .expect("empty source should error");
        assert_eq!(err.to_string(), "empty befunge program");
    }

    #[test]
    fn newline_only_program_returns_error() {
        let err = World::from_source_string("\n\n")
            .err()
            .expect("blank source should error");
        assert_eq!(err.to_string(), "empty befunge program");
    }

    #[test]
    fn random_program() {
        let width = 128;
        let height = 64;
        let world = World::from_random(width, height);
        assert_eq!(world.width(), width);
        assert_eq!(world.height(), height);
        assert_eq!(world.field.len(), height);
        for belt in &world.field {
            assert_eq!(belt.len(), width);
        }
    }
}
