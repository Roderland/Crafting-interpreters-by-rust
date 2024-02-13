mod chunk;
mod debug;
mod value;
mod vm;

use crate::chunk::{Chunk, OpCode};
use crate::value::Value::Number;
use crate::vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_const(Number(1.2), 123);
    chunk.add_const(Number(3.4), 123);
    chunk.add_code(OpCode::OpAdd, 123);
    chunk.add_const(Number(5.6), 123);
    chunk.add_code(OpCode::OpDiv, 123);
    chunk.add_code(OpCode::OpNegate, 123);
    chunk.add_code(OpCode::OpReturn, 123);

    let _ = VM::new().interpret(chunk);
}