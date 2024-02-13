use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub enum Result {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    pub chunk: Chunk,       // 静态代码块和常量
    pub ip: usize,          // 我们的程序计数器，记录字节码执行到哪了
    pub stack: Vec<Value>,  // 我们的虚拟机栈
}

impl VM {
    pub fn new() -> VM {
        VM { chunk: Chunk::new(), ip: 0, stack: vec![] }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn run(&mut self) -> Result {
        loop {
            let bytecode = self.read_bytecode();
            match bytecode {
                OpCode::OpReturn => return Result::Ok,
                OpCode::OpConstant(index) => print!("{:?}", self.read_constant(index)),
            }
        }
    }

    fn read_bytecode(&mut self) -> OpCode {
        self.ip += 1;
        self.chunk.bytecodes[self.ip - 1]
    }

    fn read_constant(&mut self, index: usize) -> Value {
        self.chunk.constants[index]
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}