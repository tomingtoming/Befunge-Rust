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

    pub fn from_source_string(source: &str) -> World {
        let mut world: Vec<Vec<u8>> = Vec::new();
        let lines: Vec<&str> = source.split('\n').collect();
        let width = lines
            .iter()
            .fold(0, |i, s| i.max(s.len()));
        
        for line in lines {
            let mut belt = Vec::from(line);
            while belt.len() != width {
                belt.push(b' ');
            }
            world.push(belt)
        }
        
        World {
            width,
            height: world.len(),
            field: world,
        }
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
        World { width, height, field: world }
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
    fn hello_world_program() {
        let src = ">              v\nv  ,,,,,\"Hello\"<\n>48*,          v\nv,,,,,,\"World!\"<\n>25*,@";
        let world = World::from_source_string(src);
        assert_eq!(world.get(0, 0), b'>');
        assert_eq!(world.get(15, 0), b'v');
        assert_eq!(world.get(15, 4), b' ');
        assert_eq!(world.get(0, 4), b'>');
        assert_eq!(world.get(16, 5), b'>');
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
