mod chip8;
use minifb;
use periodic;

<<<<<<< HEAD
||||||| parent of 6fe98ab (:poop: compile but crash)
//const SCREEN_HZ: f64 = 60.;
//const TIMER_HZ: f64 = 60.;
//const CPU_HZ: f64 = 60.;

=======
const SCREEN_HZ: f64 = 60.;
const TIMER_HZ: f64 = 60.;
const CPU_HZ: f64 = 500.;

>>>>>>> 6fe98ab (:poop: compile but crash)
fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();
    // Chip8 system
    static mut chip8: chip8::Chip8 = chip8::Chip8::init();
    unsafe {
        chip8.load_rom(std::fs::read(&args[1]).unwrap());
    }

    //static mut lockedChip: std::sync::Arc<std::sync::Mutex<chip8::Chip8>> =
    //    std::sync::Arc::new(std::sync::Mutex::new(chip8));

    // setup graphics
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

    // setup planer
    let mut planner = periodic::Planner::new();
    planner.start();

    planner.add(
        || unsafe {
            chip8.update_timer();
        },
        periodic::Every::new(std::time::Duration::from_secs_f64(1. / TIMER_HZ)),
    );

    planner.add(
        || unsafe {
            chip8.tick();
        },
        periodic::Every::new(std::time::Duration::from_secs_f64(1. / CPU_HZ)),
    );

    // GUI loop
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        //chip8.tick();
        //let _must_beep = chip8.update_timer();
        // TODO beep

        // draw graphics from chip8.gfx
<<<<<<< HEAD
        if chip8.get_draw_flag() {
            window
                .update_with_buffer(&chip8.get_gfx_buffer(), chip8::WIDTH, chip8::HEIGHT)
                .unwrap();
        }
||||||| parent of 6fe98ab (:poop: compile but crash)
        window
            .update_with_buffer(&chip8.get_gfx_buffer(), chip8::WIDTH, chip8::HEIGHT)
            .unwrap();
=======
        window
            .update_with_buffer(
                unsafe { &chip8.get_gfx_buffer() },
                chip8::WIDTH,
                chip8::HEIGHT,
            )
            .unwrap();
>>>>>>> 6fe98ab (:poop: compile but crash)

        // update key press state
        // minifb cannot detect KeyX from an azerty keyboard
        // alternative solution : AZER QSDF UIOP JKLM ?
        unsafe {
            chip8.reset_keypad();
        }
        for key in window.get_keys() {
            match key {
                minifb::Key::Key1 => unsafe { chip8.press_key(0x1) },
                minifb::Key::Key2 => unsafe { chip8.press_key(0x2) },
                minifb::Key::Key3 => unsafe { chip8.press_key(0x3) },
                minifb::Key::Key4 => unsafe { chip8.press_key(0xC) },
                minifb::Key::Q => unsafe { chip8.press_key(0x4) },
                minifb::Key::W => unsafe { chip8.press_key(0x5) },
                minifb::Key::E => unsafe { chip8.press_key(0x6) },
                minifb::Key::R => unsafe { chip8.press_key(0xD) },
                minifb::Key::A => unsafe { chip8.press_key(0x7) },
                minifb::Key::S => unsafe { chip8.press_key(0x8) },
                minifb::Key::D => unsafe { chip8.press_key(0x9) },
                minifb::Key::F => unsafe { chip8.press_key(0xE) },
                minifb::Key::Z => unsafe { chip8.press_key(0xA) },
                minifb::Key::X => unsafe { chip8.press_key(0x0) },
                minifb::Key::C => unsafe { chip8.press_key(0xB) },
                minifb::Key::V => unsafe { chip8.press_key(0xF) },
                _ => {}
            }
        }
    }
}
