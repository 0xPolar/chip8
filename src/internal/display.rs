pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub const BUFFER_SIZE: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;

pub struct Display {
    buffer: [bool; BUFFER_SIZE],
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [false; BUFFER_SIZE],
        }
    }
    // Clear the screen
    pub fn clear(&mut self) {
        self.buffer = [false; BUFFER_SIZE];
    }

    pub fn buffer(&self) -> &[bool; BUFFER_SIZE] {
        &self.buffer
    }

    pub fn draw_sprite(&mut self, x_start: usize, y_start: usize, sprite_bytes: &[u8]) -> bool {
        let mut collision = false;

        for (row, &byte) in sprite_bytes.iter().enumerate() {
            let y = (y_start + row) % DISPLAY_HEIGHT; // wrap vertically

            for col in 0..8 {
                // bit 7 is leftmost pixel, bit 0 is rightmost
                if byte & (0x80 >> col) != 0 {
                    let x = (x_start + col) % DISPLAY_WIDTH; // wrap horizontally
                    let idx = y * DISPLAY_WIDTH + x;

                    if self.buffer[idx] {
                        collision = true; // pixel was on, XOR turns it off → collision
                    }
                    self.buffer[idx] ^= true;
                }
            }
        }

        collision
    }
}
