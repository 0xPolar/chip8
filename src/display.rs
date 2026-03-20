pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub const DISPLAY_PIXELS: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;

pub struct Display {
    pixels: [bool; DISPLAY_PIXELS],
}

impl Display {
    // Clear the screen
    fn clear(&mut self) {
        self.pixels = [false; DISPLAY_PIXELS];
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        return self.pixels[y * DISPLAY_WIDTH + x];
    }

    fn draw_sprite(&mut self, x_start: usize, y_start: usize, sprite_bytes: &[u8]) -> bool {
        let mut collision = false;

        for (row, &byte) in sprite_bytes.iter().enumerate() {
            let y = (y_start + row) % DISPLAY_HEIGHT; // wrap vertically

            for col in 0..8 {
                // bit 7 is leftmost pixel, bit 0 is rightmost
                if byte & (0x80 >> col) != 0 {
                    let x = (x_start + col) % DISPLAY_WIDTH; // wrap horizontally
                    let idx = y * DISPLAY_WIDTH + x;

                    if self.pixels[idx] {
                        collision = true; // pixel was on, XOR turns it off → collision
                    }
                    self.pixels[idx] ^= true;
                }
            }
        }

        collision
    }
}
