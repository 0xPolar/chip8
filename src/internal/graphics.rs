use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::video::GLProfile;

use glow::HasContext;

use imgui;
use imgui_glow_renderer::{AutoRenderer, TextureMap};
use imgui_sdl2_support::SdlPlatform;

use super::display::Display;
use super::keypad::Keypad;

pub struct AppWindow {
    sdl: sdl2::Sdl,
    imgui: imgui::Context,

    window: sdl2::video::Window,
    platform: SdlPlatform,
    renderer: AutoRenderer,

    event_pump: sdl2::EventPump,

    _gl_context: sdl2::video::GLContext,
    texture: glow::NativeTexture,
    texture_id: imgui::TextureId,

    last_frame: std::time::Instant,
}

impl AppWindow {
    pub fn new() -> Self {
        //Initalize SDL2 video module
        let sdl = sdl2::init().expect("Failed to init SDL2");
        let mut imgui_context = imgui::Context::create();
        let video = sdl.video().expect("Failed to init SDL2 Video");

        imgui_context.set_ini_filename(None);

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

        // Load OpenGL function pointers
        let gl = unsafe {
            glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
        };

        let platform = SdlPlatform::new(&mut imgui_context);

        let mut renderer =
            AutoRenderer::new(gl, &mut imgui_context).expect("Failed to create ImGui renderer");

        // Enable VSync
        video
            .gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
            .expect("Failed to set VSync");

        // Create the CHIP-8 display texture using the renderer's GL context
        let chip8_texture = unsafe {
            let gl = renderer.gl_context();
            let texture = gl.create_texture().expect("Failed to create texture");

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            let pixels: Option<&[u8]> = None;

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                64,
                32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                pixels,
            );

            gl.bind_texture(glow::TEXTURE_2D, None);

            texture
        };

        let texture_id = renderer
            .texture_map_mut()
            .register(chip8_texture)
            .expect("Failed to register texture");

        let event_pump = sdl.event_pump().expect("Failed to create event pump");

        AppWindow {
            sdl,
            imgui: imgui_context,
            window,
            platform,
            renderer,
            _gl_context: gl_context,
            texture: chip8_texture,
            texture_id: texture_id,
            last_frame: std::time::Instant::now(),
            event_pump,
        }
    }

    pub fn upadte_texture(&self, display: &Display) {
        let mut pixels: Vec<u8> = Vec::with_capacity(64 * 32 * 4);
        for &on in display.buffer().iter() {
            if on {
                pixels.extend_from_slice(&[255, 255, 255, 255]);
            } else {
                pixels.extend_from_slice(&[0, 0, 0, 255]);
            }
        }

        let gl = self.renderer.gl_context();
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0, // mip level
                0,
                0, // x, y offset (start at top-left)
                64,
                32, // width, height
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&pixels), // ignore:
            );
            gl.bind_texture(glow::TEXTURE_2D, None);
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
}
