pub struct DebugOutput {
    enabled: bool,
}

impl DebugOutput {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}