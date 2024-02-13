mod chunk;
mod debug;
mod value;
mod vm;

use std::ops::Index;
use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_instruction;
use crate::value::Value::Number;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_const(Number(1.2), 123);
    chunk.add_code(OpCode::OpReturn, 123);
    let mut offset = 0;
    while offset < chunk.bytecodes.len() {
        offset = disassemble_instruction(&chunk, offset)
    }
}