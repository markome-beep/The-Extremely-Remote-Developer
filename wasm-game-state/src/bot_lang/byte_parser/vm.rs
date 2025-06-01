use num_traits::FromPrimitive;

use super::{
    chunk::{Chunk, OpCode},
    util::Poppable,
    value::Value,
};

pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,
    ip: usize,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[allow(unused)]
impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn interpret_chunk(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {}

    // fn read_byte(&mut self) -> u8 {
    //     self.ip += 1;
    //     self.chunk.code[self.ip - 1]
    // }

    fn run(&mut self) -> InterpretResult {
        let mut ip = self.chunk.code.iter().enumerate();
        while let Some((i, instruction)) = ip.next() {
            // #[cfg(feature = "debug_logging")]
            // {
            print!("          ");
            self.stack.iter().for_each(|i| print!("[{i}]"));
            println!();
            self.chunk.disassemble_instruction(i);
            // }
            match OpCode::from_u8(*instruction)
                .unwrap_or_else(|| panic!("BAD OP CODE {}", instruction))
            {
                OpCode::Return => {
                    if let Some(val) = self.stack.pop() {
                        println!("{val}");
                    }
                    return InterpretResult::Ok;
                }
                OpCode::Constant => {
                    let (_, i) = ip.next().unwrap();
                    let constant = self.chunk.constants[*i as usize];
                    self.stack.push(constant);
                    println!("{constant}");
                }
                OpCode::ConstantLong => todo!(),
                OpCode::Negate => {
                    let last = self.stack.len() - 1;
                    self.stack[last] *= -1.0;
                }
                OpCode::Add => {
                    if let Some((b, a)) = self.stack.pop_2() {
                        self.stack.push(a + b);
                    }
                }
                OpCode::Subtract => {
                    if let Some((b, a)) = self.stack.pop_2() {
                        self.stack.push(a - b);
                    }
                }
                OpCode::Multiply => {
                    if let Some((b, a)) = self.stack.pop_2() {
                        self.stack.push(a * b);
                    }
                }
                OpCode::Divide => {
                    if let Some((b, a)) = self.stack.pop_2() {
                        self.stack.push(a / b);
                    }
                }
            }
        }
        InterpretResult::Ok
    }
}

#[cfg(test)]
mod byte_parser_test {
    use super::*;

    #[test]
    fn basic() {
        let mut progm = Chunk::new();
        progm.write(OpCode::Return as u8, 0);

        let mut vm = VM::new();
        vm.interpret_chunk(progm);
    }

    #[test]
    fn neg() {
        let mut progm = Chunk::new();
        progm.write_constant(10.0, 0);
        progm.write(OpCode::Negate as u8, 0);
        progm.write(OpCode::Return as u8, 0);

        let mut vm = VM::new();
        vm.interpret_chunk(progm);
    }

    #[test]
    fn bin_op() {
        let mut progm = Chunk::new();
        progm.write_constant(10.0, 0);
        progm.write_constant(-12.0, 0);
        progm.write(OpCode::Add as u8, 0);

        progm.write_constant(-12.0, 0);
        progm.write_constant(20.0, 0);
        progm.write(OpCode::Divide as u8, 0);

        progm.write_constant(2.0, 0);
        progm.write(OpCode::Divide as u8, 0);

        progm.write(OpCode::Multiply as u8, 0);

        progm.write(OpCode::Return as u8, 0);

        let mut vm = VM::new();
        vm.interpret_chunk(progm);
    }
}
