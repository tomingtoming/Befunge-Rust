pub struct World {
    pub w: usize,
    pub h: usize,
    field: Vec<Vec<u8>>,
}

impl World {
    pub fn from_source_string(source: &str) -> World {
        let mut world: Vec<Vec<u8>> = Vec::new();
        let lines: Vec<&str> = source.split('\n').collect();
        let width = lines.iter().fold(0, |i, s| if i < s.len() { s.len() } else { i });
        for line in lines {
            let mut belt = Vec::from(line);
            while belt.len() != width {
                belt.push(' ' as u8)
            }
            world.push(belt)
        }
        World { w: width, h: world.len(), field: world }
    }
    pub fn from_random(w: usize, h: usize) -> World {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let mut world: Vec<Vec<u8>> = Vec::new();
        for _ in 0..h {
            let mut line = Vec::new();
            for _ in 0..w {
                line.push(rng.gen())
            }
            world.push(line)
        }
        World { w, h, field: world }
    }
    pub fn println(&self) {
        for line in &self.field {
            for elem in line {
                if 0x20 <= *elem && *elem <= 0x7e {
                    print!("{}", *elem as char);
                } else {
                    print!("{}", '□');
                }
            }
            println!();
        }
    }
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.field[y % self.h][x % self.w]
    }
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.field[y % self.h][x % self.w] = value
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn hello_world_program() {
        let src = ">              v\nv  ,,,,,\"Hello\"<\n>48*,          v\nv,,,,,,\"World!\"<\n>25*,@";
        let world = World::from_source_string(src);
        assert_eq!(world.get(0, 0) as char, '>');
        assert_eq!(world.get(15, 0) as char, 'v');
        assert_eq!(world.get(15, 4) as char, ' ');
        assert_eq!(world.get(0, 4) as char, '>');
        assert_eq!(world.get(16, 5) as char, '>');
    }

    #[test]
    fn random_program() {
        let w = 128;
        let h = 64;
        let world = World::from_random(w, h);
        assert_eq!(world.w, w);
        assert_eq!(world.h, h);
        assert_eq!(world.field.len(), h);
        for belt in &world.field {
            assert_eq!(belt.len(), w);
        }
    }
}
