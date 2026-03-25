mod internal;

use internal::chip8::Chip8;

fn read_rom(rom_path: String) -> Result<Vec<u8>, String> {
    std::fs::read(rom_path).map_err(|err| format!("Failed to Read ROM: {}", err))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_path = &args[1];

    let rom_bytes = read_rom(rom_path.to_string()).unwrap();
    let rom = &rom_bytes[..];

    let mut c8 = Chip8::new();

    let x = c8.load_rom(rom).unwrap_or_else(|err| err.to_string());
    println!("Hello, world!");
}
