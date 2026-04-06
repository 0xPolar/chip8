mod debugger;
mod internal;

use internal::audio::AudioSystem;
use internal::chip8::Chip8;
use internal::graphics::AppWindow;

fn read_rom(rom_path: String) -> Result<Vec<u8>, String> {
    std::fs::read(rom_path).map_err(|err| format!("Failed to read ROM: {}", err))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_path = &args[1];

    let rom_bytes = read_rom(rom_path.to_string()).unwrap();
    let mut c8 = Chip8::new();
    c8.load_rom(&rom_bytes)
        .unwrap_or_else(|err| err.to_string());

    let mut window = AppWindow::new();
    let mut audio = AudioSystem::new();

    let mut last_frame = std::time::Instant::now();
    loop {
        let dt = last_frame.elapsed().as_secs_f32();
        last_frame = std::time::Instant::now();

        if window.process_events(&mut c8.keypad) {
            break;
        }

        let cycles = (500.0 * dt) as usize;
        for _ in 0..cycles.max(1) {
            c8.tick();
        }

        let timer_ticks = (60.0 * dt) as usize;
        for _ in 0..timer_ticks {
            c8.tick_times();
        }
        audio.update(c8.sound_active());

        window.upadte_texture(&c8.display);
        window.clear();
        window.swap();
    }
}
