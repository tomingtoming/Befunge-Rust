use std::io;

const FUNGE_WIDTH: usize = 80;
const FUNGE_HEIGHT: usize = 25;

pub struct World {
    width: usize,
    height: usize,
    field: Vec<Vec<u8>>,
}

impl World {
    fn wrap_signed_coordinate(coord: i32, size: usize) -> usize {
        coord.rem_euclid(size as i32) as usize
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn from_source_string(source: &str) -> io::Result<World> {
        let lines: Vec<&str> = source.lines().collect();

        if lines.len() > FUNGE_HEIGHT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "program exceeds 25 rows",
            ));
        }

        if lines.iter().any(|line| line.len() > FUNGE_WIDTH) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "program exceeds 80 columns",
            ));
        }

        let mut world = vec![vec![b' '; FUNGE_WIDTH]; FUNGE_HEIGHT];
        for (y, line) in lines.into_iter().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                world[y][x] = byte;
            }
        }

        Ok(World {
            width: FUNGE_WIDTH,
            height: FUNGE_HEIGHT,
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

    pub fn get_signed(&self, x: i32, y: i32) -> u8 {
        self.get(
            Self::wrap_signed_coordinate(x, self.width),
            Self::wrap_signed_coordinate(y, self.height),
        )
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.field[y % self.height][x % self.width] = value;
    }

    pub fn set_signed(&mut self, x: i32, y: i32, value: u8) {
        let x = Self::wrap_signed_coordinate(x, self.width);
        let y = Self::wrap_signed_coordinate(y, self.height);
        self.set(x, y, value);
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
        assert_eq!(world.width(), 80);
        assert_eq!(world.height(), 25);
        assert_eq!(world.get(0, 0), b'>');
        assert_eq!(world.get(15, 0), b'v');
        assert_eq!(world.get(15, 4), b' ');
        assert_eq!(world.get(0, 5), b' ');
        assert_eq!(world.get(79, 24), b' ');
        Ok(())
    }

    #[test]
    fn source_is_loaded_into_a_fixed_80_by_25_torus() -> std::io::Result<()> {
        let world = World::from_source_string(">@\n")?;
        assert_eq!(world.width(), 80);
        assert_eq!(world.height(), 25);
        assert_eq!(world.get(0, 0), b'>');
        assert_eq!(world.get(1, 0), b'@');
        assert_eq!(world.get(2, 0), b' ');
        assert_eq!(world.get(0, 1), b' ');
        Ok(())
    }

    #[test]
    fn empty_program_creates_a_blank_torus() -> std::io::Result<()> {
        let world = World::from_source_string("")?;
        assert_eq!(world.width(), 80);
        assert_eq!(world.height(), 25);
        assert_eq!(world.get(0, 0), b' ');
        assert_eq!(world.get(79, 24), b' ');
        Ok(())
    }

    #[test]
    fn newline_only_program_creates_blank_torus() -> std::io::Result<()> {
        let world = World::from_source_string("\n\n")?;
        assert_eq!(world.width(), 80);
        assert_eq!(world.height(), 25);
        assert_eq!(world.get(0, 0), b' ');
        assert_eq!(world.get(0, 1), b' ');
        assert_eq!(world.get(79, 24), b' ');
        Ok(())
    }

    #[test]
    fn signed_coordinates_wrap_toroidally() -> std::io::Result<()> {
        let mut world = World::from_source_string("abc\ndef")?;
        world.set_signed(-1, -1, b'!');
        assert_eq!(world.get(79, 24), b'!');
        assert_eq!(world.get_signed(-1, -1), b'!');
        Ok(())
    }

    #[test]
    fn oversized_sources_are_rejected() {
        let too_wide = "x".repeat(81);
        let too_tall = (0..26).map(|_| "x").collect::<Vec<_>>().join("\n");

        let err = World::from_source_string(&too_wide)
            .err()
            .expect("wide source should error");
        assert_eq!(err.to_string(), "program exceeds 80 columns");

        let err = World::from_source_string(&too_tall)
            .err()
            .expect("tall source should error");
        assert_eq!(err.to_string(), "program exceeds 25 rows");
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
