use macroquad::prelude::*;
use crate::SCALE;

pub fn draw_chip_8_pixel(x: usize, y: usize) {
    draw_rectangle(
        x as f32 * SCALE as f32, 
        y as f32 * SCALE as f32, 
        SCALE as f32,
        SCALE as f32, 
        GRAY
    );
}
