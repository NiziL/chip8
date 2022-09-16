mod chip8;
use minifb;

fn main() {
    // setup graphics and input
    let mut window = minifb::Window::new(
        "Chip8 Emulator",
        chip8::WIDTH,
        chip8::HEIGHT,
        minifb::WindowOptions {
            scale: minifb::Scale::FitScreen,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..minifb::WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // 60fps

    // Chip8 system
    let mut chip8 = chip8::init();
    //chip8.load_rom(std::fs::read("roms/programs/IBM Logo.ch8").unwrap());
    //chip8.load_rom(std::fs::read("roms/demos/Stars [Sergey Naydenov, 2010].ch8").unwrap());
    chip8.load_rom(std::fs::read("roms/demos/Maze [David Winter, 199x].ch8").unwrap());

    // Emulation loop
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // chip8 cpu tick
        chip8.tick();
        // draw graphics from chip8.gfx
        window
            .update_with_buffer(&chip8.get_gfx_buffer(), chip8::WIDTH, chip8::HEIGHT)
            .unwrap();
        // update key press state
    }
}
