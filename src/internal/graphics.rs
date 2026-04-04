use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::video::GLProfile;

use super::keypad::Keypad;

pub struct AppWindow {
    sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,
}

impl AppWindow {
    pub fn new() -> Self {
        //Initalize SDL2 video module
        let sdl = sdl2::init().expect("Failed to init SDL2");
        let video = sdl.video().expect("Failed to init SDL2 Video");

        // Configure OpenGL version
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // Create Window
        let window = video
            .window("CHIP8", 1100, 700)
            .opengl()
            .resizable()
            .position_centered()
            .build()
            .expect("Failed to create window");

        // Create OpenGL context
        let gl_context = window
            .gl_create_context()
            .expect("Failed to create OpenGL context");

        // Enable VSync
        video
            .gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
            .expect("Failed to set VSync");

        let event_pump = sdl.event_pump().expect("Failed to create event pump");

        AppWindow {
            sdl,
            window,
            _gl_context: gl_context,
            event_pump,
        }
    }

    fn update_keypad(&mut self, keypad: &mut Keypad) {
        const KEY_MAP: [(Scancode, usize); 16] = [
            (Scancode::X, 0x0),
            (Scancode::Num1, 0x1),
            (Scancode::Num2, 0x2),
            (Scancode::Num3, 0x3),
            (Scancode::Q, 0x4),
            (Scancode::W, 0x5),
            (Scancode::E, 0x6),
            (Scancode::A, 0x7),
            (Scancode::S, 0x8),
            (Scancode::D, 0x9),
            (Scancode::Z, 0xA),
            (Scancode::C, 0xB),
            (Scancode::Num4, 0xC),
            (Scancode::R, 0xD),
            (Scancode::F, 0xE),
            (Scancode::V, 0xF),
        ];

        let keyboard = self.event_pump.keyboard_state();
        for (scancode, idx) in KEY_MAP {
            if keyboard.is_scancode_pressed(scancode) {
                keypad.press(idx);
            } else {
                keypad.release(idx);
            }
        }
    }

    pub fn process_events(&mut self, keyboard: &mut Keypad) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                _ => {}
            }
        }

        self.update_keypad(keyboard);
        false
    }

    pub fn swap(&self) {
        self.window.gl_swap_window();
    }
}
