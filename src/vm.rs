use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::vm::Result::RuntimeError;

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

macro_rules! binary_op {
    //                     let b = self.pop();
    //                     let a = self.pop();
    //                     let value = match (a, b) {
    //                         (Value::Number(a), Value::Number(b)) => {
    //                             let c = Value::Number(a + b);
    //                             self.push(c);
    //                         }
    //                     };
    ($self:ident, $op:tt) => {{
        let  b = $self.pop();
        let  a = $self.pop();
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => $self.push(Value::Number(a $op b)),
            _ => return RuntimeError,
        };
    }}
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
            // rust条件编译，是否要在虚拟机执行期间开启栈&字节码信息的打印
            #[cfg(feature = "debug_trace_execution")] {
                use crate::debug::disassemble_instruction;

                // 打印栈信息
                print!("[STACK]   ");
                for value in &self.stack {
                    print!("[{}]", value);
                }
                println!();

                // 打印字节码
                disassemble_instruction(&self.chunk, self.ip);
            }

            let bytecode = self.read_bytecode();
            match bytecode {
                OpCode::OpReturn => {
                    print!("程序执行结果：{}", self.pop());
                    return Result::Ok;
                },
                OpCode::OpConstant(index) => {
                    let constant = self.read_constant(index);
                    self.push(constant)
                },
                OpCode::OpNegate => {
                    if let Value::Number(value) = self.pop() {
                        self.push(Value::Number(-value))
                    } else {
                        panic!("never");
                    }
                },
                OpCode::OpAdd => binary_op!(self, +),
                OpCode::OpSub => binary_op!(self, -),
                OpCode::OpMul => binary_op!(self, *),
                OpCode::OpDiv => binary_op!(self, /),
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