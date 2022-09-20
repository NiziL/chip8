mod chip8;
use minifb;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time;

fn main() {
    let cpu_duration: time::Duration = time::Duration::from_secs_f32(1. / 500.);
    let timer_duration: time::Duration = time::Duration::from_secs_f64(1. / 60.);
    let win_duration: time::Duration = time::Duration::from_secs_f64(1. / 1000.);

    // get command line arguments
    let args: Vec<String> = std::env::args().collect();
    // Init chip8 system
    let chip8: chip8::Chip8 = chip8::Chip8::init(std::fs::read(&args[1]).unwrap());
    let chip8lock = Arc::new(Mutex::new(chip8));

    // Init minifb window
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
    .unwrap();

    // Init multithreading lock and channel
    let (gfx_transmitter, gfx_receiver) = mpsc::channel();
    let cpu_lock = Arc::clone(&chip8lock);
    let timer_lock = Arc::clone(&chip8lock);
    let keypad_lock = Arc::clone(&chip8lock);

    // CPU tick thread
    thread::spawn(move || loop {
        let now = time::Instant::now();

        let mut chip8 = cpu_lock.lock().unwrap();
        chip8.tick();
        if chip8.get_draw_flag() {
            gfx_transmitter.send(chip8.get_gfx_buffer()).unwrap();
        }
        drop(chip8);

        let elapsed = now.elapsed();
        if elapsed < cpu_duration {
            thread::sleep(cpu_duration - elapsed);
        } else {
            eprintln!("CPU frequency has been messed up");
        }
    });

    // Timer update thread
    thread::spawn(move || loop {
        let now = time::Instant::now();

        let mut chip8 = timer_lock.lock().unwrap();
        let _must_beep = chip8.update_timer();
        drop(chip8);
        // TODO start/stop beep

        let elapsed = now.elapsed();
        if elapsed < timer_duration {
            thread::sleep(timer_duration - elapsed);
        } else {
            eprintln!("Timers frequency has been messed up");
        }
    });

    // GUI & keypad update loop
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        if let Ok(gfx) = gfx_receiver.recv_timeout(win_duration) {
            window
                .update_with_buffer(&gfx, chip8::WIDTH, chip8::HEIGHT)
                .unwrap();
        } else {
            window.update();
        }
        let mut chip8 = keypad_lock.lock().unwrap();
        chip8.reset_keypad();
        window.get_keys().iter().for_each(|key| match key {
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
        });
        drop(chip8);
    }
}
