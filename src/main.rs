mod chip8;
use minifb;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time;

fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();
    // Chip8 system
    let chip8: chip8::Chip8 = chip8::Chip8::init(std::fs::read(&args[1]).unwrap());
    let chip8lock = Arc::new(Mutex::new(chip8));

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
    .unwrap();

    let cpu_duration = time::Duration::from_secs_f64(1. / 500.);
    let timer_duration = time::Duration::from_secs_f64(1. / 60.);

    let (gfx_transmitter, gfx_receiver) = mpsc::channel();

    let cpu_lock = Arc::clone(&chip8lock);
    thread::spawn(move || loop {
        let now = time::Instant::now();

        let mut chip8 = cpu_lock.lock().unwrap();
        println!("cpu tick");
        chip8.tick();
        if chip8.get_draw_flag() {
            gfx_transmitter.send(chip8.get_gfx_buffer()).unwrap();
        }
        drop(chip8);

        let elapsed = now.elapsed();
        if elapsed < cpu_duration {
            thread::sleep(cpu_duration - elapsed);
        } else {
            //panic!("CPU frequency cannot be respected !");
            println!("CPU freq fucked up")
        }
    });

    let timer_lock = Arc::clone(&chip8lock);
    thread::spawn(move || loop {
        let now = time::Instant::now();

        let mut chip8 = timer_lock.lock().unwrap();
        println!("timer tick");
        let _must_beep = chip8.update_timer();
        drop(chip8);
        // TODO start/stop beep

        let elapsed = now.elapsed();
        if elapsed < timer_duration {
            thread::sleep(timer_duration - elapsed);
        } else {
            //panic!("Timer frequency cannot be respected !");
            println!("Timer freq fucked up");
        }
    });

    let keypad_lock = Arc::clone(&chip8lock);
    for gfx in gfx_receiver {
        println!("receive a gfx");
        window
            .update_with_buffer(&gfx, chip8::WIDTH, chip8::HEIGHT)
            .unwrap();
        let mut chip8 = keypad_lock.lock().unwrap();
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
