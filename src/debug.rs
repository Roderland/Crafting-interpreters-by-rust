use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ")
    } else {
        print!("{:4} ", chunk.lines[offset])
    }
    let bytecode: OpCode = chunk.bytecodes[offset];
    // rust模式匹配语法，用它来翻译所有类型的指令吧
    match bytecode {
        OpCode::OpReturn => simple_instruction("OP_RETURN"),
        OpCode::OpConstant(index) =>
            const_instruction("OP_CONSTANT", index, chunk.constants[index]),
        OpCode::OpNegate => simple_instruction("OP_NEGATE"),
        OpCode::OpAdd => simple_instruction("OP_ADD"),
        OpCode::OpSub => simple_instruction("OP_SUB"),
        OpCode::OpMul => simple_instruction("OP_MUL"),
        OpCode::OpDiv => simple_instruction("OP_DIV"),
    };
    offset + 1
}

fn simple_instruction(code_name: &str) {
    println!("{}", code_name)
}

fn const_instruction(code_name: &str, index: usize, constant: Value) {
    println!("{:<16} {:4} {:?}", code_name, index, constant);
}