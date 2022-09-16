use rand::Rng;

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

    key: [bool; N_KEY],

    delay_timer: u8,
    sound_timer: u8,
}

// TODO : handles timers !

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

        key: [false; N_KEY],

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
        // check if pc overflow
        if self.pc >= MEM_SIZE as u16 {
            panic!("PC points outside of the memory")
        }

        // fetch opcode
        let op1 = (self.mem[self.pc as usize] as u16) << 8;
        let op2 = self.mem[(self.pc + 1) as usize] as u16;
        self.opcode = op1 | op2;

        // process opcode
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x0FFF {
                // 0000 - No operation.
                // not in official chip8 instruction set
                0x0000 => {
                    self.pc += 2;
                }

                // 00E0 - Clear the screen.
                0x00E0 => {
                    self.gfx.fill(false);
                    self.pc += 2;
                }

                // 00EE - Returns from a subroutine.
                0x00EE => {
                    //TODO
                }

                // 0NNN - Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN.
                // does not implements if you don't want to emulate a COSMAC VIP
                _ => panic!("0NNN not implemented (opcode {:04X})", self.opcode),
            },

            // 1NNN - Jump to address NNN.
            0x1000 => {
                self.pc = self.opcode & 0x0FFF;
            }

            // 2NNN - Calls subroutine at NNN.
            0x2000 => {
                //TODO
            }

            // 3XNN - Skips next instruction if VX equals NN.
            0x3000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let vx = self.reg[x as usize];
                let nn = self.opcode & 0x00FF;
                if vx == nn as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // 4XNN - Skips the next instruction if VX does not equals NN.
            0x4000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let vx = self.reg[x as usize];
                let nn = self.opcode & 0x00FF;
                if vx != nn as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // 5XY0 - Skips the next instruction if VX equals VY.
            0x5000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let vx = self.reg[x as usize];
                let y = (self.opcode & 0x00F0) >> 4;
                let vy = self.reg[y as usize];
                if vx == vy {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // 6XNN - Set VX to NN.
            0x6000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = self.opcode & 0x00FF;
                self.reg[x as usize] = nn as u8;
                self.pc += 2;
            }

            // 7XNN - Adds NN to VX (carry flags is not changed).
            0x7000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = self.opcode & 0x00FF;
                let (value, _) = self.reg[x as usize].overflowing_add(nn as u8);
                self.reg[x as usize] = value;
                self.pc += 2;
            }

            0x8000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let vx = self.reg[x as usize];
                let y = (self.opcode & 0x00F0) >> 4;
                let vy = self.reg[y as usize];
                match self.opcode & 0x000F {
                    // 8XY0 - Sets VX to the value of VY.
                    0x0000 => {
                        self.reg[x as usize] = vy;
                    }

                    // 8XY1 - Sets VX to VX or VY.
                    0x0001 => {
                        self.reg[x as usize] = vx | vy;
                    }

                    // 8XY2 - Sets VX to VX and VY.
                    0x0002 => {
                        self.reg[x as usize] = vx & vy;
                    }

                    // 8XY3 - Sets VX to VX xor VY.
                    0x0003 => {
                        self.reg[x as usize] = vx ^ vy;
                    }

                    // 8XY4 - Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
                    0x0004 => {
                        let (value, overflow) = vx.overflowing_add(vy);
                        self.reg[x as usize] = value;
                        self.reg[0xF] = if overflow { 1 } else { 0 };
                    }

                    // 8XY5 - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                    0x0005 => {
                        let (value, overflow) = vx.overflowing_sub(vy);
                        self.reg[x as usize] = value;
                        self.reg[0xF] = if overflow { 0 } else { 1 };
                    }

                    // 8XY6 - Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                    // !! Ambiguous instruction, instruction changed in CHIP-48 and SUPER-CHIP...
                    // !! original impl (COSMAC VIP) is VX = VY << 1, it has been changed to VX = VX << 1
                    0x0006 => {
                        self.reg[0xF] = vx & 0x01;
                        self.reg[x as usize] <<= 1;
                    }

                    // 8XY7 - Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                    0x0007 => {
                        let (value, overflow) = vy.overflowing_sub(vx);
                        self.reg[x as usize] = value;
                        self.reg[0xF] = if overflow { 0 } else { 1 };
                    }

                    // 8XYE - Stores the most significant bit of VX in VF and then shifts VX to the right by 1.
                    // !! Ambiguous instruction, instruction changed in CHIP-48 and SUPER-CHIP...
                    // !! original impl (COSMAC VIP) is VX = VY >> 1, it has been changed to VX = VX >> 1
                    0x000E => {
                        self.reg[0xF] = vx & 0x80;
                        self.reg[x as usize] >>= 1;
                    }

                    _ => panic!("Unknown opcode {:04X}", self.opcode),
                }
                self.pc += 2;
            }

            // 9XY0 - Skips the next instruction if VX does not equal VY.
            0x9000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let vx = self.reg[x as usize];
                let y = (self.opcode & 0x00F0) >> 4;
                let vy = self.reg[y as usize];
                if vx != vy {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // ANNN - Sets I to the address NNN
            0xA000 => {
                self.index = self.opcode & 0x0FFF;
                self.pc += 2;
            }

            // BNNN - Jumps to the address NNN plus V0.
            0xB000 => {
                let nnn = self.opcode & 0x0FFF;
                self.pc = nnn + self.reg[0] as u16;
            }

            // CXNN - Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
            0xC000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = (self.opcode & 0x00FF) as u8;
                let random: u8 = rand::thread_rng().gen();
                self.reg[x as usize] = random & nn;
                self.pc += 2;
            }

            // DXYN - Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a
            // height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory
            // location I; I value does not change after the execution of this instruction. VF is
            // set to 1 if any screen pixels are flipped from set to unset when the sprite is
            // drawn, and to 0 if that does not happen.
            0xD000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let vx = (self.reg[x as usize] & (WIDTH as u8 - 1)) as usize; // modulo 64
                let y = (self.opcode & 0x00F0) >> 4;
                let vy = (self.reg[y as usize] & (HEIGHT as u8 - 1)) as usize; // modulo 32
                let n = (self.opcode & 0x000F) as usize;

                self.reg[0xF] = 0;

                for i in 0..n {
                    let src_pixel = self.mem[self.index as usize + i];
                    for j in 0..8 {
                        // clip sprite drawn outside the screen
                        if vx + j >= WIDTH || vy + i >= HEIGHT {
                            continue;
                        }

                        // if current pixel is not 0
                        if (src_pixel & (0x80 >> j)) != 0 {
                            let dst = (vx + j) + (vy + i) * WIDTH;
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

            0xE000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let vx = self.reg[x as usize];
                match self.opcode & 0x00FF {
                    // EX9E - Skips the next instruction if the key stored in VX is pressed.
                    0x009E => {
                        if self.key[vx as usize] {
                            self.pc += 2;
                        }
                        self.pc += 2
                    }

                    // EXA1 - Skips the next instruction if the key stored in VX is not pressed.
                    0x00A1 => {
                        if !self.key[vx as usize] {
                            self.pc += 2;
                        }
                        self.pc += 2
                    }
                    _ => panic!("unknown opcode {:04X}", self.opcode),
                }
            }

            0xF000 => {
                let x = (self.opcode & 0x0F00) >> 8;
                match self.opcode & 0x00FF {
                    // FX07 - Sets VX to the value of the delay timer.
                    0x0007 => {
                        self.reg[x as usize] = self.delay_timer;
                    }

                    // FX0A - A key press is awaited, and then stored in VX.
                    0x000A => {
                        // TODO, don't know how I will implements this
                    }

                    // FX15 - Sets the delay timer to VX.
                    0x0015 => self.delay_timer = self.reg[x as usize],

                    // FX18 - Sets the sound timer to VX.
                    0x0018 => self.sound_timer = self.reg[x as usize],

                    // FX1E - Adds VX to I. VF is not affected.
                    0x001E => self.index += self.reg[x as usize] as u16,

                    // FX29 - Sets I to the location of the sprite for the character in VX.
                    0x0029 => {
                        let vx = self.reg[x as usize] as u16;
                        self.index = 0x050 + (vx * 5);
                    }

                    // FX33 - Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2.
                    0x0033 => {
                        let vx = self.reg[x as usize];
                        self.mem[self.index as usize] = vx / 100;
                        self.mem[(self.index + 1) as usize] = (vx / 10) % 10;
                        self.mem[(self.index + 2) as usize] = (vx % 100) / 10;
                    }

                    // FX55 - Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.
                    0x0055 => {
                        for i in 0..x {
                            self.mem[(self.index + i) as usize] = self.reg[i as usize];
                        }
                    }

                    // FX65 - Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.
                    0x0065 => {
                        for i in 0..x {
                            self.reg[i as usize] = self.mem[(self.index + i) as usize];
                        }
                    }

                    _ => panic!("unknown opcode {:04X}", self.opcode),
                }
                self.pc += 2;
            }

            // wildcard to panic
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
