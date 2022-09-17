mod chip8;
use minifb;

fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();
    // Chip8 system
    let mut chip8 = chip8::init();
    chip8.load_rom(std::fs::read(&args[1]).unwrap());

    // setup graphics and input
    let mut window = minifb::Window::new(
        "Chip8 Emulator",
        chip8::WIDTH,
        chip8::HEIGHT,
        minifb::WindowOptions {
            scale: minifb::Scale::X8,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..minifb::WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // GUI loop
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        chip8.tick();
        let _must_beep = chip8.update_timer();
        // TODO beep

        // draw graphics from chip8.gfx
        if chip8.get_draw_flag() {
            window
                .update_with_buffer(&chip8.get_gfx_buffer(), chip8::WIDTH, chip8::HEIGHT)
                .unwrap();
        }

        // update key press state
        // minifb cannot detect KeyX from an azerty keyboard
        // alternative solution : AZER QSDF UIOP JKLM ?
        chip8.reset_keypad();
        for key in window.get_keys() {
            match key {
                minifb::Key::Key1 => chip8.press_key(0x1),
                minifb::Key::Key2 => chip8.press_key(0x2),
                minifb::Key::Key3 => chip8.press_key(0x3),
                minifb::Key::Key4 => chip8.press_key(0xC),
                minifb::Key::Q => chip8.press_key(0x4),
                minifb::Key::W => chip8.press_key(0x5),
                minifb::Key::E => chip8.press_key(0x6),
                minifb::Key::R => chip8.press_key(0xD),
                minifb::Key::A => chip8.press_key(0x7),
                minifb::Key::S => chip8.press_key(0x8),
                minifb::Key::D => chip8.press_key(0x9),
                minifb::Key::F => chip8.press_key(0xE),
                minifb::Key::Z => chip8.press_key(0xA),
                minifb::Key::X => chip8.press_key(0x0),
                minifb::Key::C => chip8.press_key(0xB),
                minifb::Key::V => chip8.press_key(0xF),
                _ => {}
            }
        }
    }
}
