use crate::value::Value;

#[derive(Copy, Clone)]
pub enum OpCode {
    OpReturn,
    OpConstant(usize),
}

pub struct Chunk {
    pub(crate) bytecodes: Vec<OpCode>,
    pub(crate) constants: Vec<Value>,
    pub(crate) lines: Vec<usize>,
}

// rust的面向对象语法，在impl代码块中定义struct的成员方法
impl Chunk {
    // 我们用new方法管理Chunk的构造
    pub fn new() -> Chunk {
        // vec![...]是rust提供的一个宏，new一个vec很方便吧
        Chunk{ bytecodes: vec![], constants: vec![], lines: vec![] }
    }

    pub fn add_code(&mut self, code: OpCode, line: usize) {
        self.bytecodes.push(code);
        self.lines.push(line);
        // lines中记录每个字节码对应在源代码中的行数，两者长度一定是相同的
        assert_eq!(self.bytecodes.len(), self.lines.len());
    }

    pub fn add_const(&mut self, constant: Value, line: usize) {
        // index记录一下常量在常量池里的索引
        let index = self.constants.len();
        self.constants.push(constant);
        // 偷懒直接把索引写到字节码里☺️
        self.add_code(OpCode::OpConstant(index), line)
    }
}