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
}
