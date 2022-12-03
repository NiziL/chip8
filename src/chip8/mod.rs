pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

mod opcodes;
mod processor;
pub use processor::{init, Chip8};
