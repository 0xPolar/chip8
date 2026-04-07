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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_clear() {
        let mut display = Display::new();
        display.draw_sprite(0, 0, &[0xFF]);
        display.clear();
        for y in 0..32 {
            for x in 0..64 {
                assert!(!display.get_pixel(x, y));
            }
        }
    }

    #[test]
    fn single_pixel_draw() {
        let mut display = Display::new();
        display.draw_sprite(0, 0, &[0x80]); // 10000000
        assert!(display.get_pixel(0, 0));
        assert!(!display.get_pixel(1, 0));
    }

    #[test]
    fn xor_collision() {
        let mut display = Display::new();
        let collision1 = display.draw_sprite(0, 0, &[0x80]);
        assert!(!collision1);
        let collision2 = display.draw_sprite(0, 0, &[0x80]);
        assert!(collision2);
        assert!(!display.get_pixel(0, 0)); // XOR turned it back off
    }

    #[test]
    fn screen_wrapping() {
        let mut display = Display::new();
        display.draw_sprite(62, 0, &[0xFF]); // starts at x=62, wraps
        assert!(display.get_pixel(62, 0));
        assert!(display.get_pixel(63, 0));
        assert!(display.get_pixel(0, 0)); // wrapped
        assert!(display.get_pixel(1, 0)); // wrapped
    }
}
