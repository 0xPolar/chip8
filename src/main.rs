mod debugger;
mod internal;

use internal::audio::AudioSystem;
use internal::chip8::Chip8;
use internal::graphics::AppWindow;

fn read_rom(rom_path: String) -> Result<Vec<u8>, String> {
    std::fs::read(rom_path).map_err(|err| format!("Failed to read ROM: {}", err))
}

fn main() {}
