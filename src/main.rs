mod internal;

use internal::chip8::Chip8;
use internal::graphics::GraphicsWindow;

fn read_rom(rom_path: String) -> Result<Vec<u8>, String> {
    std::fs::read(rom_path).map_err(|err| format!("Failed to Read ROM: {}", err))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_path = &args[1];

    let rom_bytes = read_rom(rom_path.to_string()).unwrap();
    let rom = &rom_bytes[..];

    let mut c8 = Chip8::new();

    c8.load_rom(rom).unwrap_or_else(|err| err.to_string());

    let mut window = GraphicsWindow::new();

    while !window.should_close() {
        let dt = window.get_frame_time();

        window.update_keypad(&mut c8.keypad);

        let cycles = (500.0 * dt) as usize;
        for _ in 0..cycles.max(1) {
            c8.tick();
        }

        let timer_ticks = (60.0 * dt) as usize;
        for _ in 0..timer_ticks {
            c8.tick_times();
        }

        window.draw(&c8.display);
    }
}
