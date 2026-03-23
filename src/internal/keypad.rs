pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    fn is_pressed(&self, idx: usize) -> bool {
        return self.keys[idx];
    }

    fn press(&mut self, idx: usize) {
        self.keys[idx] = true;
    }

    fn release(&mut self, idx: usize) {
        self.keys[idx] = false;
    }
}
