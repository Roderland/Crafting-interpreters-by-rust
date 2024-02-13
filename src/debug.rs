use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    let bytecode: OpCode = chunk.bytecodes[offset];
    // rust模式匹配语法，用它来翻译所有类型的指令吧
    match bytecode {
        OpCode::OpReturn => simple_instruction("OP_RETURN"),
        OpCode::OpConstant(index) =>
            const_instruction("OP_CONSTANT", index, chunk.constants[index]),
    };
    offset + 1
}

fn simple_instruction(code_name: &str) {
    println!("{}", code_name)
}

fn const_instruction(code_name: &str, index: usize, constant: Value) {
    println!("{:<16} {:4} {:?}", code_name, index, constant);
}