pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
    pub fn is_pressed(&self, idx: usize) -> bool {
        return self.keys[idx];
    }

    pub fn press(&mut self, idx: usize) {
        self.keys[idx] = true;
    }

    pub fn release(&mut self, idx: usize) {
        self.keys[idx] = false;
    }
}
