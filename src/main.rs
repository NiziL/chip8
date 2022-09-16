mod chip8;
use minifb;

const SCREEN_HZ: f64 = 60.;
const TIMER_HZ: f64 = 60.;
const CPU_HZ: f64 = 60.;

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
            scale: minifb::Scale::X16,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..minifb::WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));
    window.limit_update_rate(Some(std::time::Duration::from_secs_f64(1. / SCREEN_HZ)));

    // GUI loop
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        chip8.tick();
        let must_beep = chip8.update_timer();
        // TODO beep

        // draw graphics from chip8.gfx
        window
            .update_with_buffer(&chip8.get_gfx_buffer(), chip8::WIDTH, chip8::HEIGHT)
            .unwrap();

        // update key press state
        // cannot detect &é" from my azerty keyboard...
        // alternative solution : AZER QSDF UIOP JKLM
        chip8.reset_keypad();
        for key in window.get_keys() {
            match key {
                minifb::Key::A => chip8.press_key(0),
                minifb::Key::Z => chip8.press_key(1),
                minifb::Key::E => chip8.press_key(2),
                minifb::Key::R => chip8.press_key(3),
                minifb::Key::Q => chip8.press_key(4),
                minifb::Key::S => chip8.press_key(5),
                minifb::Key::D => chip8.press_key(6),
                minifb::Key::F => chip8.press_key(7),
                minifb::Key::U => chip8.press_key(8),
                minifb::Key::I => chip8.press_key(9),
                minifb::Key::O => chip8.press_key(10),
                minifb::Key::P => chip8.press_key(11),
                minifb::Key::J => chip8.press_key(12),
                minifb::Key::K => chip8.press_key(13),
                minifb::Key::L => chip8.press_key(14),
                minifb::Key::M => chip8.press_key(15),
                _ => {}
            }
        }
    }
}
