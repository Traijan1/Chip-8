use std::{thread::sleep, time::Duration};

use macroquad::{file, miniquad::window::set_window_size, prelude::*};
use rodio::{DeviceSinkBuilder, Player, Source, play, source::SquareWave};

use crate::{emulator::{chip8::Chip8, framebuffer::{FRAMEBUFFER_HEIGHT, FRAMEBUFFER_WIDTH}}, frontend::{graphic::draw_chip_8_pixel, keyboard::keys_down}};

mod emulator;
mod frontend;

const SCALE: usize = 10;
const TICK_RATE: f32 = 1.0 / 60.0;
const CYCLES_PER_TICK: usize = 11; // ca. 660 Befehle/Sekunde
const MAX_ACCUMULATED: f32 = 0.25; // nach Pausen max. 15 Ticks nachholen

#[macroquad::main("Chip-8")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sink = DeviceSinkBuilder::open_default_sink()?;
    let wave = SquareWave::new(440.).amplify(0.05);
    let player = Player::connect_new(sink.mixer());
    player.append(wave);
    player.pause();

    set_window_size((FRAMEBUFFER_WIDTH * SCALE) as u32, (FRAMEBUFFER_HEIGHT * SCALE) as u32);
    let file = file::load_file("roms/5-quirks.ch8").await?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&file)?;

    let mut accumulator = 0.0;

    loop {
        accumulator += get_frame_time();
        accumulator = accumulator.min(MAX_ACCUMULATED);

        if chip8.sound_timer() > 0 && player.is_paused() {
                player.play();
        }
        else if chip8.sound_timer() == 0 && !player.is_paused() {
            player.pause();
        }

        while accumulator >= TICK_RATE {
            chip8.keypad.set_keys(keys_down());

            for _ in 0..CYCLES_PER_TICK {
                chip8.step()?;
            }

            chip8.tick_timers();
            accumulator -= TICK_RATE;
        }

        clear_background(BLACK);

        for x in 0..FRAMEBUFFER_WIDTH {
            for y in 0..FRAMEBUFFER_HEIGHT {
                if chip8.framebuffer.is_pixel_on(x, y) {
                    draw_chip_8_pixel(x, y);
                }
            }
        }

        next_frame().await
    }
}