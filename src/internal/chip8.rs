use crate::internal::{cpu::CPU, display::Display, font::FONT_DATA, keypad::Keypad};

struct Chip8 {
    memory: [u8; 4096],
    cpu: CPU,
    display: Display,
    keypad: Keypad,
}

impl Chip8 {
    fn new() -> Self {
        let mut chip8 = Self {
            memory: [0x00; 4096],
            cpu: CPU::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        };

        chip8.memory[0x050..0x050 + FONT_DATA.len()].copy_from_slice(&FONT_DATA);

        chip8
    }

    fn load_rom(&mut self, rom_path: String) -> Result<String, String> {
        let rom_bytes = match std::fs::read(rom_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Failed to read ROM: {}", e)),
        };

        if rom_bytes.len() > (4096 - 0x050) {
            return Err("Rom Too Large!!!".to_string());
        };

        self.memory[0x200..0x200 + rom_bytes.len()].copy_from_slice(&rom_bytes);

        Ok("Loaded Rom Into memory".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_memory_state() {
        let chip8 = Chip8::new();

        assert_eq!(chip8.memory, [0x00; 4096])
    }

    #[test]
    fn font_loaded_in_memory() {
        let chip8 = Chip8::new();

        assert_eq!(chip8.memory[0x050..0x050 + FONT_DATA.len()], FONT_DATA)
    }

    #[test]
    fn rom_loading() {
        let mut chip8 = Chip8::new();

        chip8.load_rom("test.ch8".to_string()).unwrap();
    }
}
