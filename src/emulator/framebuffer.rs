pub const FRAMEBUFFER_WIDTH: usize = 64;
pub const FRAMEBUFFER_HEIGHT: usize = 32;
const BUFFER_SIZE: usize = FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT;

pub struct Framebuffer {
    buffer: [bool; BUFFER_SIZE]
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            buffer: [false; BUFFER_SIZE]
        }
    }

    pub fn is_pixel_on(&self, x: usize, y: usize) -> bool {
        // TODO: Check for out of bound access

        let index = y * FRAMEBUFFER_WIDTH + x;
        self.buffer[index]
    }

    pub(super) fn clear_buffer(&mut self) {
        self.buffer.fill(false);
    }

    pub(super) fn draw(&mut self, x: u8, y: u8, to_draw: &[u8]) -> u8 {
        let mut collision = 0;
        let mask = 0x80;

        let start_x = x as usize % FRAMEBUFFER_WIDTH;
        let start_y = y as usize % FRAMEBUFFER_HEIGHT;

        for (row, column) in to_draw.iter().enumerate() {
            for i in 0..8 {
                let active_pixel = (column & (mask >> i)) != 0;
                let screen_x = start_x + i;
                let screen_y = start_y + row;

                if screen_x >= FRAMEBUFFER_WIDTH || screen_y >= FRAMEBUFFER_HEIGHT {
                    continue;
                }

                let index = screen_y * FRAMEBUFFER_WIDTH + screen_x;

                if active_pixel && self.buffer[index] {
                    collision = 1;
                }

                self.buffer[index] ^= active_pixel;
            }
        }

        collision
    }
}