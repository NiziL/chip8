# chip8

Yet another Chip8 emulator written in Rust, powered by [minifb](https://github.com/emoon/rust_minifb) and [rand](https://github.com/rust-random/rand).

Many thanks to Tobias V. Langhoff for [his great blog post](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/), and to jellysquid3 for [the inspiration and the fontset](https://github.com/jellysquid3/chip8-rs).

## Usage

`$ cargo run [path_to_rom]`

press ESC to close the window.

A git submodule refering to [a chip8 roms collection](https://github.com/kripod/chip8-roms) is provided for convenience at `roms/`.

## Keypad

The original chip8 keypad is mapped on 1234QWERASDFZXCV, as usual for chip8 emulator.  
**It won't work if your keyboard layout is not a qwerty.**

## Misc

The opcodes 8XY6, 8XYE, FX55 and FX65 slightly differs depending on the implementations. 
This emulator stick to the CHIP-48 version.  
More information on [wikipedia](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table). 