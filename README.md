# Chip-8

A small [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in Rust. 
Built as a **learning project** to get comfortable with the language.

CHIP-8 is the classic "first emulator": a tiny virtual machine with 35 opcodes, 4 KB of memory and a 64×32 monochrome display. Small enough to finish, big enough to teach you a lot about ownership, enums, pattern matching, error handling and project structure in Rust.

## Features

- Full CPU with fetch, decode, execute loop
- 64×32 display, scaled up 10× via [macroquad](https://github.com/not-fl3/macroquad)
- 60 Hz delay & sound timers, with a beep powered by [rodio](https://github.com/RustAudio/rodio)
- 16-key hex keypad mapped to your keyboard
- Typed errors with [thiserror](https://github.com/dtolnay/thiserror)

## Executing

```sh
cargo run --release
```

The ROM to load is currently set in `src/main.rs`. A few test ROMs live in `roms/`.

## ⌨Controls

```
CHIP-8 keypad      Keyboard
1 2 3 C            1 2 3 4
4 5 6 D     =>     Q W E R
7 8 9 E            A S D F
A 0 B F            Y X C V
```

## Project layout

```
src/
├── main.rs          # window, game loop, timing & audio
├── emulator/        # the actual CHIP-8 machine
│   ├── chip8.rs     # ties everything together
│   ├── cpu.rs       # registers & opcode execution
│   ├── instruction.rs
│   ├── memory.rs
│   ├── framebuffer.rs
│   ├── keypad.rs
│   └── error.rs
└── frontend/        # macroquad rendering & input
```

## Resources

- [Tobias V. Langhoff — Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Timendus' CHIP-8 test suite](https://github.com/Timendus/chip8-test-suite) (source of the test ROMs)
- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
