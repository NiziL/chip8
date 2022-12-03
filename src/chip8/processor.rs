use super::opcodes::InstructionSet;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const N_REG: usize = 16;
const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const N_KEY: usize = 16;

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

    pub fn keypad(&self) -> [bool; N_KEY] {
        self.key
    }

    pub fn reset_keypad(&mut self) {
        self.key.fill(false);
    }

    pub fn press_key(&mut self, key: usize) {
        self.key[key] = true;
    }

    pub fn is_key_down(&self, key: usize) -> bool {
        self.key[key]
    }

    pub fn gfx(&self, position: usize) -> bool {
        self.gfx[position]
    }

    pub fn set_gfx(&mut self, position: usize, value: bool) {
        self.gfx[position] = value;
    }

    pub fn set_draw_flag(&mut self) {
        self.draw_flag = true;
    }

    pub fn reset_gfx(&mut self) {
        self.gfx.fill(false);
        self.draw_flag = false;
    }

    pub fn gfx_buffer(&mut self) -> Vec<u32> {
        self.draw_flag = false;
        self.gfx
            .into_iter()
            .map(|x| if x { 0xFFFFFFFF } else { 0 })
            .collect()
    }

    pub fn draw_flag(&self) -> bool {
        self.draw_flag
    }

    pub fn update_timer(&mut self) -> bool {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        self.sound_timer > 0
    }

    pub fn delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

    pub fn stack_pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack is empty")
        }
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn stack(&mut self) {
        if self.sp == STACK_SIZE as u16 {
            panic!("Stack is full")
        }
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn next_instruction(&mut self) {
        self.pc += 2;
    }

    pub fn register(&self, x: usize) -> u8 {
        self.reg[x]
    }

    pub fn set_register(&mut self, x: usize, nn: u8) {
        self.reg[x] = nn;
    }

    pub fn set_index(&mut self, value: u16) {
        self.index = value;
    }

    pub fn add_index(&mut self, value: u16) {
        self.index += value;
    }

    pub fn set_memory_at_index(&mut self, offset: usize, value: u8) {
        self.mem[self.index as usize + offset] = value;
    }

    pub fn memory_at_index(&self, offset: usize) -> u8 {
        self.mem[self.index as usize + offset]
    }

    pub fn copy_n_reg_to_mem_from_index(&mut self, n: usize) {
        let i = self.index as usize;
        self.mem[i..=(i + n)].copy_from_slice(&self.reg[0..=n]);
        self.index += n as u16 + 1;
    }

    pub fn copy_mem_from_index_to_n_reg(&mut self, n: usize) {
        let i = self.index as usize;
        self.reg[0..=n].copy_from_slice(&self.mem[i..=(i + n)]);
        self.index += n as u16 + 1;
    }

    pub fn tick(&mut self) {
        // check if pc overflow
        if self.pc >= MEM_SIZE as u16 {
            panic!("PC points outside of the memory");
        }

        // fetch opcode
        let opcode_u8 = (self.mem[self.pc as usize], self.mem[self.pc as usize + 1]);
        let opcode_u4 = (
            opcode_u8.0 >> 4,
            opcode_u8.0 & 0x0F,
            opcode_u8.1 >> 4,
            opcode_u8.1 & 0x0F,
        );
        let x = opcode_u4.1 as usize;
        let y = opcode_u4.2 as usize;
        let n = opcode_u4.3;
        let nn = opcode_u8.1;
        let nnn = ((opcode_u8.0 & 0x0F) as u16) << 8 | opcode_u8.1 as u16;

        match opcode_u4 {
            // 00E0 - Clear the screen.
            (0x00, 0x00, 0x0e, 0x00) => self.process_00e0(),
            // 00EE - Returns from a subroutine.
            (0x00, 0x00, 0x0e, 0x0e) => self.process_00ee(),
            // 0NNN - Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN.
            (0x00, _, _, _) => panic!("RCA 1802 not implemented."),
            // 1NNN - Jump to address NNN.
            (0x01, _, _, _) => self.process_1nnn(nnn),
            // 2NNN - Calls subroutine at NNN.
            (0x02, _, _, _) => self.process_2nnn(nnn),
            // 3XNN - Skips next instruction if VX equals NN.
            (0x03, _, _, _) => self.process_3xnn(x, nn),
            // 4XNN - Skips the next instruction if VX does not equals NN.
            (0x04, _, _, _) => self.process_4xnn(x, nn),
            // 5XY0 - Skips the next instruction if VX equals VY.
            (0x05, _, _, 0x00) => self.process_5xy0(x, y),
            // 6XNN - Set VX to NN.
            (0x06, _, _, _) => self.process_6xnn(x, nn),
            // 7XNN - Adds NN to VX (VF is not changed).
            (0x07, _, _, _) => self.process_7xnn(x, nn),
            // 8XY0 - Sets VX to the value of VY.
            (0x08, _, _, 0x00) => self.process_8xy0(x, y),
            // 8XY1 - Sets VX to VX or VY.
            (0x08, _, _, 0x01) => self.process_8xy1(x, y),
            // 8XY2 - Sets VX to VX and VY.
            (0x08, _, _, 0x02) => self.process_8xy2(x, y),
            // 8XY3 - Sets VX to VX xor VY.
            (0x08, _, _, 0x03) => self.process_8xy3(x, y),
            // 8XY4 - Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
            (0x08, _, _, 0x04) => self.process_8xy4(x, y),
            // 8XY5 - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
            (0x08, _, _, 0x05) => self.process_8xy5(x, y),
            // 8XY6 - Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
            (0x08, _, _, 0x06) => self.process_8xy6(x, y),
            // 8XY7 - Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
            (0x08, _, _, 0x07) => self.process_8xy7(x, y),
            // 8XYE - Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
            (0x08, _, _, 0x0e) => self.process_8xye(x, y),
            // 9XY0 - Skips the next instruction if VX does not equal VY.
            (0x09, _, _, 0x00) => self.process_9xy0(x, y),
            // ANNN - Sets I to the address NNN
            (0x0a, _, _, _) => self.process_annn(nnn),
            // BNNN - Jumps to the address NNN plus V0.
            (0x0b, _, _, _) => self.process_bnnn(nnn),
            // CXNN - Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
            (0x0c, _, _, _) => self.process_cxnn(x, nn),
            // DXYN - Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a
            // height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory
            // location I; I value does not change after the execution of this instruction. VF is
            // set to 1 if any screen pixels are flipped from set to unset when the sprite is
            // drawn, and to 0 if that does not happen.
            (0x0d, _, _, _) => self.process_dxyn(x, y, n),
            // EX9E - Skips the next instruction if the key stored in VX is pressed.
            (0x0e, _, 0x09, 0x0e) => self.process_ex9e(x),
            // EXA1 - Skips the next instruction if the key stored in VX is not pressed.
            (0x0e, _, 0x0a, 0x01) => self.process_exa1(x),
            // FX07 - Sets VX to the value of the delay timer.
            (0x0f, _, 0x00, 0x07) => self.process_fx07(x),
            // FX0A - A key press is awaited, and then stored in VX.
            (0x0f, _, 0x00, 0x0a) => self.process_fx0a(x),
            // FX15 - Sets the delay timer to VX.
            (0x0f, _, 0x01, 0x05) => self.process_fx15(x),
            // FX18 - Sets the sound timer to VX.
            (0x0f, _, 0x01, 0x08) => self.process_fx18(x),
            // FX1E - Adds VX to I. VF is not affected.
            (0x0f, _, 0x01, 0x0e) => self.process_fx1e(x),
            // FX29 - Sets I to the location of the sprite for the character in VX.
            (0x0f, _, 0x02, 0x09) => self.process_fx29(x),
            // FX33 - Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2.
            (0x0f, _, 0x03, 0x03) => self.process_fx33(x),
            // FX55 - Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written.
            (0x0f, _, 0x05, 0x05) => self.process_fx55(x),
            // FX65 - Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read.
            (0x0f, _, 0x06, 0x05) => self.process_fx65(x),
            // Unknown opcode
            _ => panic!("Unknown opcode {:02X}{:02X}", opcode_u8.0, opcode_u8.1),
        }
    }
}
