use super::{Chip8, HEIGHT, WIDTH};
use rand::Rng;

pub trait InstructionSet {
    fn process_00e0(&mut self);
    fn process_00ee(&mut self);
    fn process_1nnn(&mut self, nnn: u16);
    fn process_2nnn(&mut self, nnn: u16);
    fn process_3xnn(&mut self, x: usize, nn: u8);
    fn process_4xnn(&mut self, x: usize, nn: u8);
    fn process_5xy0(&mut self, x: usize, y: usize);
    fn process_6xnn(&mut self, x: usize, nn: u8);
    fn process_7xnn(&mut self, x: usize, nn: u8);
    fn process_8xy0(&mut self, x: usize, y: usize);
    fn process_8xy1(&mut self, x: usize, y: usize);
    fn process_8xy2(&mut self, x: usize, y: usize);
    fn process_8xy3(&mut self, x: usize, y: usize);
    fn process_8xy4(&mut self, x: usize, y: usize);
    fn process_8xy5(&mut self, x: usize, y: usize);
    fn process_8xy6(&mut self, x: usize, y: usize);
    fn process_8xy7(&mut self, x: usize, y: usize);
    fn process_8xye(&mut self, x: usize, y: usize);
    fn process_9xy0(&mut self, x: usize, y: usize);
    fn process_annn(&mut self, nnn: u16);
    fn process_bnnn(&mut self, nnn: u16);
    fn process_cxnn(&mut self, x: usize, nn: u8);
    fn process_dxyn(&mut self, x: usize, y: usize, n: u8);
    fn process_ex9e(&mut self, x: usize);
    fn process_exa1(&mut self, x: usize);
    fn process_fx07(&mut self, x: usize);
    fn process_fx0a(&mut self, x: usize);
    fn process_fx15(&mut self, x: usize);
    fn process_fx18(&mut self, x: usize);
    fn process_fx1e(&mut self, x: usize);
    fn process_fx29(&mut self, x: usize);
    fn process_fx33(&mut self, x: usize);
    fn process_fx55(&mut self, x: usize);
    fn process_fx65(&mut self, x: usize);
}

impl InstructionSet for Chip8 {
    fn process_00e0(&mut self) {
        self.reset_gfx();
        self.next_instruction();
    }

    fn process_00ee(&mut self) {
        let i = self.stack_pop();
        self.set_program_counter(i);
        self.next_instruction();
    }

    fn process_1nnn(&mut self, nnn: u16) {
        self.set_program_counter(nnn);
    }

    fn process_2nnn(&mut self, nnn: u16) {
        self.stack();
        self.set_program_counter(nnn);
    }

    fn process_3xnn(&mut self, x: usize, nn: u8) {
        if self.register(x) == nn {
            self.next_instruction();
        }
        self.next_instruction();
    }

    fn process_4xnn(&mut self, x: usize, nn: u8) {
        if self.register(x) != nn {
            self.next_instruction();
        }
        self.next_instruction();
    }

    fn process_5xy0(&mut self, x: usize, y: usize) {
        if self.register(x) == self.register(y) {
            self.next_instruction();
        }
        self.next_instruction();
    }

    fn process_6xnn(&mut self, x: usize, nn: u8) {
        self.set_register(x, nn);
        self.next_instruction();
    }

    fn process_7xnn(&mut self, x: usize, nn: u8) {
        let (value, _) = self.register(x).overflowing_add(nn);
        self.set_register(x, value);
        self.next_instruction();
    }

    fn process_8xy0(&mut self, x: usize, y: usize) {
        self.set_register(x, self.register(y));
        self.next_instruction();
    }

    fn process_8xy1(&mut self, x: usize, y: usize) {
        self.set_register(x, self.register(x) | self.register(y));
        self.next_instruction();
    }

    fn process_8xy2(&mut self, x: usize, y: usize) {
        self.set_register(x, self.register(x) & self.register(y));
        self.next_instruction();
    }

    fn process_8xy3(&mut self, x: usize, y: usize) {
        self.set_register(x, self.register(x) ^ self.register(y));
        self.next_instruction();
    }

    fn process_8xy4(&mut self, x: usize, y: usize) {
        let (value, overflow) = self.register(x).overflowing_add(self.register(y));
        self.set_register(x, value);
        self.set_register(0xF, overflow as u8);
        self.next_instruction();
    }

    fn process_8xy5(&mut self, x: usize, y: usize) {
        let (value, overflow) = self.register(x).overflowing_sub(self.register(y));
        self.set_register(x, value);
        self.set_register(0xF, overflow as u8);
        self.next_instruction();
    }

    // Ambiguous instruction, some implementations use VX = VY >> 1, some use VX >>= 1
    fn process_8xy6(&mut self, x: usize, _y: usize) {
        self.set_register(0xF, self.register(x) & 0x01);
        self.set_register(x, self.register(x) >> 1);
        //self.set_register(x, self.register(y) >> 1);
        self.next_instruction();
    }

    fn process_8xy7(&mut self, x: usize, y: usize) {
        let (value, overflow) = self.register(y).overflowing_sub(self.register(x));
        self.set_register(x, value);
        self.set_register(0xF, if overflow { 0 } else { 1 });
        self.next_instruction();
    }

    // Ambiguous instruction, some implementations use VX = VY << 1, some use VX <<= 1
    fn process_8xye(&mut self, x: usize, _y: usize) {
        self.set_register(0xF, self.register(x) & 0x80);
        self.set_register(x, self.register(x) << 1);
        //self.set_register(x, self.register(y) << 1);
        self.next_instruction();
    }

    fn process_9xy0(&mut self, x: usize, y: usize) {
        if self.register(x) != self.register(y) {
            self.next_instruction();
        }
        self.next_instruction();
    }

    fn process_annn(&mut self, nnn: u16) {
        self.set_index(nnn);
        self.next_instruction();
    }

    fn process_bnnn(&mut self, nnn: u16) {
        self.set_program_counter(nnn + self.register(0) as u16)
    }

    fn process_cxnn(&mut self, x: usize, nn: u8) {
        let random: u8 = rand::thread_rng().gen();
        self.set_register(x, random & nn);
        self.next_instruction();
    }

    fn process_dxyn(&mut self, x: usize, y: usize, n: u8) {
        let vx = (self.register(x) & (WIDTH as u8 - 1)) as usize;
        let vy = (self.register(y) & (WIDTH as u8 - 1)) as usize;
        self.set_register(0xF, 0);

        for i in 0..n as usize {
            let src_pixel = self.memory_at_index(i);
            for j in 0..8 {
                // clip sprite drawn outside the screen
                if vx + j >= WIDTH || vy + i >= HEIGHT {
                    continue;
                }
                // if current pixel is not 0
                if (src_pixel & (0x80 >> j)) != 0 {
                    let dst = (vx + j) + (vy + i) * WIDTH;
                    // collision: set VF flag to 1
                    if self.gfx(dst) {
                        self.set_register(0xF, 1);
                    }
                    // set current pixel on framebuffer
                    self.set_gfx(dst, self.gfx(dst) ^ true);
                }
            }
        }
        self.set_draw_flag();
        self.next_instruction();
    }

    fn process_ex9e(&mut self, x: usize) {
        if self.is_key_down(self.register(x) as usize) {
            self.next_instruction();
        }
        self.next_instruction();
    }

    fn process_exa1(&mut self, x: usize) {
        if !self.is_key_down(self.register(x) as usize) {
            self.next_instruction();
        }
        self.next_instruction();
    }

    fn process_fx07(&mut self, x: usize) {
        self.set_register(x, self.delay_timer());
        self.next_instruction();
    }

    fn process_fx0a(&mut self, x: usize) {
        if let Some(i) = self.keypad().iter().position(|&v| v) {
            self.set_register(x, i as u8);
            self.next_instruction();
        }
        // stay in place to await, no pc increments
    }

    fn process_fx15(&mut self, x: usize) {
        self.set_delay_timer(self.register(x));
        self.next_instruction();
    }

    fn process_fx18(&mut self, x: usize) {
        self.set_sound_timer(self.register(x));
        self.next_instruction();
    }

    fn process_fx1e(&mut self, x: usize) {
        self.add_index(self.register(x) as u16);
        self.next_instruction();
    }

    fn process_fx29(&mut self, x: usize) {
        self.set_index(0x0050 + (self.register(x) as u16) * 5);
        self.next_instruction();
    }

    fn process_fx33(&mut self, x: usize) {
        self.set_memory_at_index(0, self.register(x) / 100);
        self.set_memory_at_index(1, (self.register(x) / 10) % 10);
        self.set_memory_at_index(2, self.register(x) % 10);
        self.next_instruction();
    }

    // !! Ambiguous instruction, some implementations left I inchanged, some left I incremented.
    fn process_fx55(&mut self, x: usize) {
        self.copy_n_reg_to_mem_from_index(x);
        self.next_instruction();
    }

    // !! Ambiguous instruction, some implementations left I inchanged, some left I incremented.
    fn process_fx65(&mut self, x: usize) {
        self.copy_mem_from_index_to_n_reg(x);
        self.next_instruction();
    }
}
