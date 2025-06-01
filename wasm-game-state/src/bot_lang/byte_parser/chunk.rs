use core::panic;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::bot_lang::byte_parser::value::Value;

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
    ConstantLong,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Chunk {
    pub code: Vec<u8>,
    lines: Vec<usize>,
    pub constants: Vec<Value>,
}

#[allow(unused)]
impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, code: u8, line: usize) {
        self.code.push(code);
        self.add_line(line, 1);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        self.constants.push(value);
        match self.constants.len() - 1 {
            c if c <= u8::MAX as usize => {
                self.code.push(OpCode::Constant as u8);
                self.code.push(c as u8);
                self.add_line(line, 2);
            }
            c if c < u32::MAX as usize => {
                self.code.push(OpCode::ConstantLong as u8);
                let bytes = (c as u32).to_le_bytes();
                self.code.extend_from_slice(&bytes);
                self.add_line(line, 5);
            }
            _ => panic!("You done fucked with the lang"),
        }
    }

    fn add_line(&mut self, line: usize, count: usize) {
        let len = self.lines.len();
        if len > 0 && line == self.lines[len - 2] {
            self.lines[len - 1] += count;
        } else {
            self.lines.push(line);
            self.lines.push(count);
        }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if let Some(line) = self.line_number(offset) {
            print!("{line:4} ")
        } else {
            print!("   | ")
        }

        match OpCode::from_u8(self.code[offset])
            .unwrap_or_else(|| panic!("BAD OP CODE {}", self.code[offset]))
        {
            OpCode::Return => Self::simple_instruction("OP_RETURN", offset),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::ConstantLong => self.constant_long_instruction("OP_CONSTANT_LONG", offset),
            OpCode::Negate => Self::simple_instruction("OP_NEGATE", offset),
            OpCode::Add => Self::simple_instruction("OP_ADD", offset),
            OpCode::Subtract => Self::simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => Self::simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => Self::simple_instruction("OP_DIVIDE", offset),
        }
    }

    fn line_number(&self, offset: usize) -> Option<usize> {
        if offset == 0 {
            return Some(self.lines[0]);
        }

        let mut current_count = 0;
        let mut rle_iter = self.lines.chunks_exact(2); // Iterate through pairs of [value, count]

        while let Some([value, count]) = rle_iter.next() {
            if current_count + count > offset {
                if current_count == offset {
                    return Some(*value);
                }
                return None;
            } else {
                current_count += count;
            }
        }
        panic!("This is a problem");
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        return offset + 1;
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{:<16} {:4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        return offset + 2;
    }

    fn constant_long_instruction(&self, name: &str, offset: usize) -> usize {
        let constant =
            u32::from_le_bytes(self.code[(offset + 1)..(offset + 5)].try_into().unwrap());

        println!(
            "{:<16} {:4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        return offset + 5;
    }
}

#[cfg(test)]
mod byte_parser_test {
    use super::*;

    #[test]
    fn basic() {
        let mut progm = Chunk::new();
        progm.write(OpCode::Return as u8, 0);
        assert_eq!(progm.lines, vec![0, 1]);

        progm.disassemble("test");
    }

    #[test]
    fn constant() {
        let mut progm = Chunk::new();
        progm.write_constant(8.0, 0);
        progm.write_constant(270., 0);
        progm.write_constant(10.1, 0);
        progm.write(OpCode::Return as u8, 9);

        assert_eq!(progm.lines, vec![0, 6, 9, 1]);
        progm.disassemble("test");
    }

    // #[test]
    // fn long_constant() {
    //     let mut progm = Chunk::new();
    //     for _ in 0..256 {
    //         progm.write_constant(8.0, 0);
    //     }
    //     progm.write_constant(270., 0);
    //     progm.write_constant(10.1, 0);
    //     progm.write(OpCode::Return as u8, 9);
    //
    //     progm.disassemble("test");
    // }
}
