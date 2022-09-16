pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const N_REG: usize = 16;
pub const MEM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const N_KEY: usize = 16;

#[derive(Debug)]
pub struct Chip8 {
    /*
    MEMORY MAP:
        0x000-0x1FF - Chip 8 interpreter (unused when emulated)
        0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
        0x200-0xFFF - Program ROM and work RAM
    */
    mem: [u8; MEM_SIZE],
    reg: [u8; N_REG],
    stack: [u16; STACK_SIZE],

    opcode: u16,
    index: u16,
    pc: u16,
    sp: u16,

    gfx: [bool; WIDTH * HEIGHT],

    key: [u8; N_KEY],

    delay_timer: u8,
    sound_timer: u8,
}

pub fn init() -> Chip8 {
    let chip8 = Chip8 {
        mem: [0; MEM_SIZE],
        reg: [0; N_REG],
        stack: [0; STACK_SIZE],

        opcode: 0,
        index: 0,
        pc: 0,
        sp: 0,

        gfx: [false; WIDTH * HEIGHT],

        key: [0; N_KEY],

        delay_timer: 0,
        sound_timer: 0,
    };

    // TODO load fontset in mem, between 0x050 and 0x0A0
    return chip8;
}

impl Chip8 {
    pub fn load_rom(&mut self, bytes: Vec<u8>) {
        let start = 0x0200;

        if bytes.len() > self.mem.len() - start {
            panic!("ROM is too big")
        }

        self.pc = start as u16;
        self.index = 0x0;
        self.stack = [0u16; 16];
        self.sp = 0x0;

        for i in 0..bytes.len() {
            self.mem[i + start] = bytes[i]
        }
    }

    pub fn tick(&mut self) {
        if self.pc >= MEM_SIZE as u16 {
            panic!("PC points outside of the memory")
        }
        // fetch opcode
        self.opcode =
            (self.mem[self.pc as usize] as u16) << 8 | (self.mem[(self.pc + 1) as usize] as u16);
        match self.opcode & 0xF000 {
            // ...
            0x0000 => {
                match self.opcode & 0x0FFF {
                    // 0000 - no op
                    0x0000 => {
                        self.pc += 2;
                    }
                    // 00E0 - Clear framebuffer
                    0x00E0 => {
                        self.gfx.fill(false);
                        self.pc += 2;
                    }
                    // 00EE - Returns from subroutine
                    0x00EE => {
                        //TODO
                    }
                    _ => panic!("Unknown opcode {:04X}", self.opcode),
                }
            }

            // 1NNN - Jump to address NNN
            0x1000 => {
                self.pc = self.opcode & 0x0FFF;
            }

            // 6XNN - Set register X to NN
            0x6000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = self.opcode & 0x00FF;
                self.reg[x as usize] = nn as u8;
                self.pc += 2;
            }

            // 7XNN - Add value NN to register X
            0x7000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = self.opcode & 0x00FF;
                //let (value, _) = self.reg[x as usize].overflowing_add(nn as u8);
                //self.reg[x as usize] = value;
                self.reg[x as usize] += nn as u8;
                self.pc += 2;
            }

            // ANNN - Sets I to NNN
            0xA000 => {
                self.index = self.opcode & 0x0FFF;
                self.pc += 2;
            }

            // DXYN - Draw a N pixels tall sprite from the memory location at I, at the horizontal
            // coordinate stored in register X and vertical coordinate stored in register Y
            0xD000 => {
                let vx = (self.opcode & 0x0F00) >> 8;
                let x = (self.reg[vx as usize] & (WIDTH as u8 - 1)) as usize; // modulo 64
                let vy = (self.opcode & 0x00F0) >> 4;
                let y = (self.reg[vy as usize] & (HEIGHT as u8 - 1)) as usize; // modulo 32
                let n = (self.opcode & 0x000F) as usize;

                self.reg[0xF] = 0;

                for i in 0..n {
                    let src_pixel = self.mem[self.index as usize + i];
                    for j in 0..8 {
                        // clip sprite drawn outside the screen
                        if x + j >= WIDTH || y + i >= HEIGHT {
                            continue;
                        }

                        // if current pixel is not 0
                        if (src_pixel & (0x80 >> j)) != 0 {
                            let dst = (x + j) + (y + i) * WIDTH;
                            // collision: set VF flag to 1
                            if self.gfx[dst] {
                                self.reg[0xF] = 1;
                            }
                            // set current pixel on framebuffer
                            self.gfx[dst] ^= true;
                        }
                    }
                }
                self.pc += 2;
            }

            // ...
            _ => panic!("unknown opcode {:04X}", self.opcode),
        }
    }

    pub fn get_gfx_buffer(&self) -> Vec<u32> {
        return self
            .gfx
            .into_iter()
            .map(|x| if x { 0xFFFFFFFF } else { 0 })
            .collect();
    }
}
