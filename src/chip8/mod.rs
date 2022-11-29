mod opcodes;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const N_REG: usize = 16;
pub const MEM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const N_KEY: usize = 16;

const START_ROM: usize = 0x0200;
const START_FONT: usize = 0x0050;
const END_FONT: usize = 0x00A0;

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

    index: u16,
    pc: u16,
    sp: u16,

    gfx: [bool; WIDTH * HEIGHT],
    draw_flag: bool,

    key: [bool; N_KEY],

    delay_timer: u8,
    sound_timer: u8,
}

pub fn init() -> Chip8 {
    let mut chip8 = Chip8 {
        mem: [0; MEM_SIZE],
        reg: [0; N_REG],
        stack: [0; STACK_SIZE],

        index: 0,
        pc: START_ROM as u16,
        sp: 0,

        gfx: [false; WIDTH * HEIGHT],
        draw_flag: false,

        key: [false; N_KEY],

        delay_timer: 0,
        sound_timer: 0,
    };
    // load fontset
    chip8.mem[START_FONT..END_FONT].copy_from_slice(include_bytes!("fontset.bin"));
    return chip8;
}

impl Chip8 {
    pub fn load_rom(&mut self, bytes: Vec<u8>) {
        if bytes.len() > self.mem.len() - START_ROM {
            panic!("ROM is too big");
        }
        self.mem[START_ROM..START_ROM + bytes.len()].copy_from_slice(&bytes);
    }

    pub fn reset(&mut self) {
        self.mem.fill(0);
        self.mem[START_FONT..END_FONT].copy_from_slice(include_bytes!("fontset.bin"));
        self.pc = START_ROM as u16;
        self.index = 0;
        self.sp = 0;
        self.reset_gfx();
        self.reset_keypad();
        self.sound_timer = 0;
        self.delay_timer = 0;
    }

    pub fn reset_keypad(&mut self) {
        self.key.fill(false);
    }

    pub fn reset_gfx(&mut self) {
        self.gfx.fill(false);
        self.draw_flag = false;
    }

    pub fn press_key(&mut self, key: usize) {
        self.key[key] = true;
    }

    pub fn gfx_buffer(&mut self) -> Vec<u32> {
        self.draw_flag = false;
        return self
            .gfx
            .into_iter()
            .map(|x| if x { 0xFFFFFFFF } else { 0 })
            .collect();
    }

    pub fn draw_flag(&self) -> bool {
        return self.draw_flag;
    }

    pub fn update_timer(&mut self) -> bool {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        return self.sound_timer > 0;
    }
    pub fn tick(&mut self) {
        // check if pc overflow
        if self.pc >= MEM_SIZE as u16 {
            panic!("PC points outside of the memory");
        }

        // fetch opcode
        let opcode = u16::from_be_bytes(
            self.mem[self.pc as usize..(self.pc + 2) as usize]
                .try_into()
                .unwrap(),
        );

        // process opcode
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x0FFF {
                // 00E0 - Clear the screen.
                0x00E0 => opcodes::process_00e0(self),
                // 00EE - Returns from a subroutine.
                0x00EE => opcodes::process_00ee(self),
                // 0NNN - Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN.
                _ => panic!("0NNN not implemented (opcode {:04X})", opcode),
            },
            // 1NNN - Jump to address NNN.
            0x1000 => opcodes::process_1nnn(self, opcode & 0x0FFF),
            // 2NNN - Calls subroutine at NNN.
            0x2000 => opcodes::process_2nnn(self, opcode & 0x0FFF),
            // 3XNN - Skips next instruction if VX equals NN.
            0x3000 => opcodes::process_3xnn(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                (opcode & 0x00FF) as u8,
            ),
            // 4XNN - Skips the next instruction if VX does not equals NN.
            0x4000 => opcodes::process_4xnn(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                (opcode & 0x00FF) as u8,
            ),
            // 5XY0 - Skips the next instruction if VX equals VY.
            0x5000 => opcodes::process_5xy0(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                ((opcode & 0x00F0) >> 4) as usize,
            ),
            // 6XNN - Set VX to NN.
            0x6000 => opcodes::process_6xnn(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                (opcode & 0x00FF) as u8,
            ),
            // 7XNN - Adds NN to VX (VF is not changed).
            0x7000 => opcodes::process_7xnn(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                (opcode & 0x00FF) as u8,
            ),
            0x8000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                match opcode & 0x000F {
                    // 8XY0 - Sets VX to the value of VY.
                    0x0000 => opcodes::process_8xy0(self, x, y),
                    // 8XY1 - Sets VX to VX or VY.
                    0x0001 => opcodes::process_8xy1(self, x, y),
                    // 8XY2 - Sets VX to VX and VY.
                    0x0002 => opcodes::process_8xy2(self, x, y),
                    // 8XY3 - Sets VX to VX xor VY.
                    0x0003 => opcodes::process_8xy3(self, x, y),
                    // 8XY4 - Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
                    0x0004 => opcodes::process_8xy4(self, x, y),
                    // 8XY5 - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                    0x0005 => opcodes::process_8xy5(self, x, y),
                    // 8XY6 - Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                    0x0006 => opcodes::process_8xy6(self, x, y),
                    // 8XY7 - Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                    0x0007 => opcodes::process_8xy7(self, x, y),
                    // 8XYE - Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                    0x000E => opcodes::process_8xye(self, x, y),
                    // Unknown opcode
                    _ => panic!("Unknown opcode {:04X}", opcode),
                }
            }
            // 9XY0 - Skips the next instruction if VX does not equal VY.
            0x9000 => opcodes::process_9xy0(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                ((opcode & 0x00F0) >> 4) as usize,
            ),
            // ANNN - Sets I to the address NNN
            0xA000 => opcodes::process_annn(self, opcode & 0x0FFF),
            // BNNN - Jumps to the address NNN plus V0.
            0xB000 => opcodes::process_bnnn(self, opcode & 0x0FFF),
            // CXNN - Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
            0xC000 => opcodes::process_cxnn(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                (opcode & 0x0FF) as u8,
            ),
            // DXYN - Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a
            // height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory
            // location I; I value does not change after the execution of this instruction. VF is
            // set to 1 if any screen pixels are flipped from set to unset when the sprite is
            // drawn, and to 0 if that does not happen.
            0xD000 => opcodes::process_dxyn(
                self,
                ((opcode & 0x0F00) >> 8) as usize,
                ((opcode & 0x00F0) >> 4) as usize,
                (opcode & 0x000F) as usize,
            ),
            0xE000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    // EX9E - Skips the next instruction if the key stored in VX is pressed.
                    0x009E => opcodes::process_ex9e(self, x),
                    // EXA1 - Skips the next instruction if the key stored in VX is not pressed.
                    0x00A1 => opcodes::process_exa1(self, x),
                    // Unknown opcode
                    _ => panic!("unknown opcode {:04X}", opcode),
                }
            }
            0xF000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    // FX07 - Sets VX to the value of the delay timer.
                    0x0007 => opcodes::process_fx07(self, x),
                    // FX0A - A key press is awaited, and then stored in VX.
                    0x000A => opcodes::process_fx0a(self, x),
                    // FX15 - Sets the delay timer to VX.
                    0x0015 => opcodes::process_fx15(self, x),
                    // FX18 - Sets the sound timer to VX.
                    0x0018 => opcodes::process_fx18(self, x),
                    // FX1E - Adds VX to I. VF is not affected.
                    0x001E => opcodes::process_fx1e(self, x),
                    // FX29 - Sets I to the location of the sprite for the character in VX.
                    0x0029 => opcodes::process_fx29(self, x),
                    // FX33 - Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2.
                    0x0033 => opcodes::process_fx33(self, x),
                    // FX55 - Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written.
                    0x0055 => opcodes::process_fx55(self, x),
                    // FX65 - Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read.
                    0x0065 => opcodes::process_fx65(self, x),
                    // Unknown opcode
                    _ => panic!("unknown opcode {:04X}", opcode),
                }
            }
            // Unknown opcode
            _ => panic!("unknown opcode {:04X}", opcode),
        }
        // increment PC (opcode is two words)
        self.pc += 2;
    }
}
