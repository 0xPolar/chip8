use crate::internal::{cpu::CPU, display::Display, font::FONT_DATA, keypad::Keypad};

pub struct Chip8 {
    memory: [u8; 4096],
    cpu: CPU,
    pub display: Display,
    pub keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0x00; 4096],
            cpu: CPU::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        };

        chip8.memory[0x050..0x050 + FONT_DATA.len()].copy_from_slice(&FONT_DATA);

        chip8
    }

    pub fn load_rom(&mut self, rom_bytes: &[u8]) -> Result<String, String> {
        if rom_bytes.len() > (4096 - 0x050) {
            return Err("Rom Too Large!!!".to_string());
        };

        self.memory[0x200..0x200 + rom_bytes.len()].copy_from_slice(&rom_bytes);

        Ok("Loaded Rom Into memory".to_string())
    }

    pub fn tick(&mut self) {
        let opcode = self.cpu.fetch(&self.memory);
        self.cpu
            .execute(opcode, &mut self.memory, &mut self.display, &self.keypad);
    }

    pub fn tick_times(&mut self) {
        self.cpu.decrement_timers();
    }

    pub fn sound_active(&self) -> bool {
        self.cpu.sound_timer() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_loaded_in_memory() {
        let chip8 = Chip8::new();
        assert_eq!(&chip8.memory[0x050..0x050 + 80], &FONT_DATA);
    }

    #[test]
    fn rom_loading() {
        let mut chip8 = Chip8::new();
        chip8.load_rom(&[0x12, 0x34, 0x56]);
        assert_eq!(chip8.memory[0x200], 0x12);
        assert_eq!(chip8.memory[0x201], 0x34);
        assert_eq!(chip8.memory[0x202], 0x56);
    }

    #[test]
    fn rom_preserves_font() {
        let mut chip8 = Chip8::new();
        chip8.load_rom(&[0xFF; 100]);
        assert_eq!(&chip8.memory[0x050..0x050 + 80], &FONT_DATA);
    }
}
