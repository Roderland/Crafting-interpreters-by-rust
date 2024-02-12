use std::ops::Index;
use crate::Value::Number;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_const(Number(1.2));
    chunk.add_code(OpCode::OpReturn);
    let mut offset = 0;
    while offset < chunk.bytecodes.len() {
        offset = disassemble_instruction(&chunk, offset)
    }
}

#[derive(Copy, Clone)]
enum OpCode {
    OpReturn,
    OpConstant(usize),
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Number(f64),
}

struct Chunk {
    bytecodes: Vec<OpCode>,
    constants: Vec<Value>,
}

// rust的面向对象语法，在impl代码块中定义struct的成员方法
impl Chunk {
    // 我们用new方法管理Chunk的构造
    fn new() -> Chunk {
        // vec![...]是rust提供的一个宏，new一个vec很方便吧
        Chunk{ bytecodes: vec![], constants: vec![] }
    }

    fn add_code(&mut self, code: OpCode) {
        self.bytecodes.push(code)
    }

    fn add_const(&mut self, constant: Value) {
        // index记录一下常量在常量池里的索引
        let index = self.constants.len();
        self.constants.push(constant);
        // 偷懒直接把索引写到字节码里☺️
        self.add_code(OpCode::OpConstant(index))
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
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


