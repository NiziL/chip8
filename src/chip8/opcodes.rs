use crate::chip8::{Chip8, HEIGHT, WIDTH};
use rand::Rng;

pub fn process_00e0(chip8: &mut Chip8) {
    chip8.reset_gfx();
}

pub fn process_00ee(chip8: &mut Chip8) {
    if chip8.sp == 0 {
        panic!("Stack is empty, cannot return from subroutine");
    }
    chip8.sp -= 1;
    chip8.pc = chip8.stack[chip8.sp as usize];
}

pub fn process_1nnn(chip8: &mut Chip8, nnn: u16) {
    chip8.pc = nnn;
    chip8.pc -= 2;
}

pub fn process_2nnn(chip8: &mut Chip8, nnn: u16) {
    chip8.stack[chip8.sp as usize] = chip8.pc;
    chip8.sp += 1;
    chip8.pc = nnn;
    chip8.pc -= 2;
}

pub fn process_3xnn(chip8: &mut Chip8, x: usize, nn: u8) {
    if chip8.reg[x] == nn {
        chip8.pc += 2;
    }
}

pub fn process_4xnn(chip8: &mut Chip8, x: usize, nn: u8) {
    if chip8.reg[x] != nn {
        chip8.pc += 2;
    }
}

pub fn process_5xy0(chip8: &mut Chip8, x: usize, y: usize) {
    if chip8.reg[x] == chip8.reg[y] {
        chip8.pc += 2;
    }
}

pub fn process_6xnn(chip8: &mut Chip8, x: usize, nn: u8) {
    chip8.reg[x] = nn;
}

pub fn process_7xnn(chip8: &mut Chip8, x: usize, nn: u8) {
    let (value, _) = chip8.reg[x].overflowing_add(nn);
    chip8.reg[x] = value;
}

pub fn process_8xy0(chip8: &mut Chip8, x: usize, y: usize) {
    chip8.reg[x] = chip8.reg[y];
}

pub fn process_8xy1(chip8: &mut Chip8, x: usize, y: usize) {
    chip8.reg[x] = chip8.reg[x] | chip8.reg[y];
}

pub fn process_8xy2(chip8: &mut Chip8, x: usize, y: usize) {
    chip8.reg[x] = chip8.reg[x] & chip8.reg[y];
}

pub fn process_8xy3(chip8: &mut Chip8, x: usize, y: usize) {
    chip8.reg[x] = chip8.reg[x] ^ chip8.reg[y];
}

pub fn process_8xy4(chip8: &mut Chip8, x: usize, y: usize) {
    let (value, overflow) = chip8.reg[x].overflowing_add(chip8.reg[y]);
    chip8.reg[x] = value;
    chip8.reg[0xF] = overflow as u8;
}

pub fn process_8xy5(chip8: &mut Chip8, x: usize, y: usize) {
    let (value, overflow) = chip8.reg[x].overflowing_sub(chip8.reg[y]);
    chip8.reg[x] = value;
    chip8.reg[0xF] = !overflow as u8;
}

// Ambiguous instruction, some implementations use VX = VY >> 1, some use VX >>= 1
pub fn process_8xy6(chip8: &mut Chip8, x: usize, y: usize) {
    chip8.reg[0xF] = chip8.reg[x] & 0x01;
    chip8.reg[x] >>= 1;
    //chip8.reg[x] = chip8.reg[y] >> 1;
}

pub fn process_8xy7(chip8: &mut Chip8, x: usize, y: usize) {
    let (value, overflow) = chip8.reg[y].overflowing_sub(chip8.reg[x]);
    chip8.reg[x] = value;
    chip8.reg[0xF] = if overflow { 0 } else { 1 };
}

// Ambiguous instruction, some implementations use VX = VY << 1, some use VX <<= 1
pub fn process_8xye(chip8: &mut Chip8, x: usize, y: usize) {
    chip8.reg[0xF] = chip8.reg[x] & 0x80;
    chip8.reg[x] <<= 1;
    //chip8.reg[x] = chip8.reg[y] << 1;
}

pub fn process_9xy0(chip8: &mut Chip8, x: usize, y: usize) {
    if chip8.reg[x] != chip8.reg[y] {
        chip8.pc += 2;
    }
}

pub fn process_annn(chip8: &mut Chip8, nnn: u16) {
    chip8.index = nnn;
}

pub fn process_bnnn(chip8: &mut Chip8, nnn: u16) {
    chip8.pc = nnn + chip8.reg[0] as u16;
    chip8.pc -= 2;
}

pub fn process_cxnn(chip8: &mut Chip8, x: usize, nn: u8) {
    let random: u8 = rand::thread_rng().gen();
    chip8.reg[x] = random & nn;
}

pub fn process_dxyn(chip8: &mut Chip8, x: usize, y: usize, n: usize) {
    let vx = (chip8.reg[x] & (WIDTH as u8 - 1)) as usize;
    let vy = (chip8.reg[y] & (WIDTH as u8 - 1)) as usize;
    chip8.reg[0xF] = 0;

    for i in 0..n {
        let src_pixel = chip8.mem[chip8.index as usize + i];
        for j in 0..8 {
            // clip sprite drawn outside the screen
            if vx + j >= WIDTH || vy + i >= HEIGHT {
                continue;
            }
            // if current pixel is not 0
            if (src_pixel & (0x80 >> j)) != 0 {
                let dst = (vx + j) + (vy + i) * WIDTH;
                // collision: set VF flag to 1
                if chip8.gfx[dst] {
                    chip8.reg[0xF] = 1;
                }
                // set current pixel on framebuffer
                chip8.gfx[dst] ^= true;
            }
        }
    }
    chip8.draw_flag = true;
}

pub fn process_ex9e(chip8: &mut Chip8, x: usize) {
    if chip8.key[chip8.reg[x] as usize] {
        chip8.pc += 2;
    }
}

pub fn process_exa1(chip8: &mut Chip8, x: usize) {
    if !chip8.key[chip8.reg[x] as usize] {
        chip8.pc += 2;
    }
}

pub fn process_fx07(chip8: &mut Chip8, x: usize) {
    chip8.reg[x] = chip8.delay_timer;
}

pub fn process_fx0a(chip8: &mut Chip8, x: usize) {
    if let Some(i) = chip8.key.iter().position(|&v| v) {
        chip8.reg[x] = i as u8;
    } else {
        // stay in place to await
        chip8.pc -= 2;
    }
}

pub fn process_fx15(chip8: &mut Chip8, x: usize) {
    chip8.delay_timer = chip8.reg[x];
}

pub fn process_fx18(chip8: &mut Chip8, x: usize) {
    chip8.sound_timer = chip8.reg[x];
}

pub fn process_fx1e(chip8: &mut Chip8, x: usize) {
    chip8.index += chip8.reg[x] as u16;
}

pub fn process_fx29(chip8: &mut Chip8, x: usize) {
    chip8.index = 0x0050 + (chip8.reg[x] as u16) * 5;
}

pub fn process_fx33(chip8: &mut Chip8, x: usize) {
    let i = chip8.index as usize;
    chip8.mem[i] = chip8.reg[x] / 100;
    chip8.mem[i + 1] = (chip8.reg[x] / 10) % 10;
    chip8.mem[i + 2] = chip8.reg[x] % 10;
}

// !! Ambiguous instruction, some implementations left I inchanged, some left I incremented.
pub fn process_fx55(chip8: &mut Chip8, x: usize) {
    let i = chip8.index as usize;
    chip8.mem[i..=(i + x)].copy_from_slice(&chip8.reg[0..=x]);
    chip8.index += x as u16 + 1;
}

// !! Ambiguous instruction, some implementations left I inchanged, some left I incremented.
pub fn process_fx65(chip8: &mut Chip8, x: usize) {
    let i = chip8.index as usize;
    chip8.reg[0..=x].copy_from_slice(&chip8.mem[i..=(i + x)]);
    chip8.index += x as u16 + 1;
}
