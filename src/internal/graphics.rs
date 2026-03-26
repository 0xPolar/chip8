use super::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display};
use super::keypad::Keypad;
use raylib::prelude::*;

const SCALE: i32 = 10;

pub struct GraphicsWindow {
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl GraphicsWindow {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(DISPLAY_WIDTH as i32 * SCALE, DISPLAY_HEIGHT as i32 * SCALE)
            .title("CHIP-8")
            .build();

        rl.set_target_fps(60);

        GraphicsWindow {
            rl: rl,
            thread: thread,
        }
    }

    pub fn should_close(&self) -> bool {
        self.rl.window_should_close()
    }

    pub fn draw(&mut self, display: &Display) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                if display.get_pixel(x, y) {
                    d.draw_rectangle(
                        x as i32 * SCALE,
                        y as i32 * SCALE,
                        SCALE,
                        SCALE,
                        Color::WHITE,
                    );
                }
            }
        }
    }

    pub fn update_keypad(&self, keypad: &mut Keypad) {
        const KEY_MAP: [(KeyboardKey, usize); 16] = [
            (KeyboardKey::KEY_X, 0x0),
            (KeyboardKey::KEY_ONE, 0x1),
            (KeyboardKey::KEY_TWO, 0x2),
            (KeyboardKey::KEY_THREE, 0x3),
            (KeyboardKey::KEY_Q, 0x4),
            (KeyboardKey::KEY_W, 0x5),
            (KeyboardKey::KEY_E, 0x6),
            (KeyboardKey::KEY_A, 0x7),
            (KeyboardKey::KEY_S, 0x8),
            (KeyboardKey::KEY_D, 0x9),
            (KeyboardKey::KEY_Z, 0xA),
            (KeyboardKey::KEY_C, 0xB),
            (KeyboardKey::KEY_FOUR, 0xC),
            (KeyboardKey::KEY_R, 0xD),
            (KeyboardKey::KEY_F, 0xE),
            (KeyboardKey::KEY_V, 0xF),
        ];

        for (key, idx) in KEY_MAP {
            if self.rl.is_key_down(key) {
                keypad.press(idx);
            } else {
                keypad.release(idx);
            }
        }
    }

    pub fn get_frame_time(&self) -> f32 {
        self.rl.get_frame_time()
    }
}
