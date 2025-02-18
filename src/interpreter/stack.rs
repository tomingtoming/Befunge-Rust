use std::fmt;

pub struct Stack {
    values: Vec<i64>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: i64) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> i64 {
        self.values.pop().unwrap_or(0)
    }

    pub fn peek(&self) -> Option<&i64> {
        self.values.last()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn swap_top(&mut self) {
        if self.len() >= 2 {
            let len = self.values.len();
            self.values.swap(len - 1, len - 2);
        }
    }

    pub fn duplicate_top(&mut self) {
        if let Some(&value) = self.peek() {
            self.push(value);
        }
    }

    // Format stack value as string with hex and ASCII representation
    fn format_value(value: i64) -> String {
        let hex = format!("0x{:02X}", value);
        if (0x20..=0x7E).contains(&value) {
            format!("{} ('{}')", hex, char::from_u32(value as u32).unwrap())
        } else {
            hex
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let formatted: Vec<_> = self
            .values
            .iter()
            .map(|&v| Stack::format_value(v))
            .collect();
        write!(f, "{}]", formatted.join(", "))
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
