mod chunk;
mod debug;
mod value;
mod vm;

use std::ops::Index;
use crate::chunk::{Chunk, OpCode};
use crate::value::Value::Number;
use crate::vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_const(Number(1.2), 123);
    chunk.add_code(OpCode::OpReturn, 123);
    let _ = VM::new().interpret(chunk);
}