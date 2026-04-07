mod debugger;
mod internal;

use debugger::state::DebugState;
use internal::audio::AudioSystem;
use internal::chip8::Chip8;
use internal::graphics::AppWindow;

use std::time::{Duration, Instant};

const TARGET_FPS: f64 = 60.0;

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
    let mut debug_state = DebugState::new();

    let reset_rom = rom_bytes.clone();
    let frame_duration = Duration::from_secs_f64(1.0 / TARGET_FPS);

    loop {
        let frame_start = Instant::now();
        if window.process_events(&mut c8.keypad) {
            break;
        }

        debug_state.update(&c8);

        let tex_id = window.chip8_texture_id();

        let dt = window.frame(&c8.display, |ui| {
            debugger::ui::draw_all(ui, &mut debug_state, tex_id);
        });

        if debug_state.reset_requested {
            c8 = Chip8::new();
            c8.load_rom(&reset_rom)
                .unwrap_or_else(|err| err.to_string());
            debug_state.reset_requested = false;
        }

        if debug_state.should_execute() {
            let cycles = (500.0 * dt) as usize;
            for _ in 0..cycles.max(1) {
                c8.tick();

                if debug_state.breakpoints.contains(&c8.program_counter()) {
                    debug_state.paused = true;
                    break;
                }
            }

            debug_state.step_requested = false;
        }

        let timer_ticks = (60.0 * dt) as usize;
        for _ in 0..timer_ticks {
            c8.tick_times();
        }
        audio.update(c8.sound_active());

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}
