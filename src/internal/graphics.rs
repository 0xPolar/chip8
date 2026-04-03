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
}
