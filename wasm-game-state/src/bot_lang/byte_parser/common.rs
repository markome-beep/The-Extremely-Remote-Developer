enum OpCode {
    OpReturn,
}

type Chunk = Vec<OpCode>;

fn disassemble_chunk(c: Chunk, name: &str) {
    println!("== {name} ==");

    let mut offset = 0;
    while offset < c.len() {
        offset = disassemble_instruction(&c, offset);
    }
}

fn disassemble_instruction(c: &Chunk, offset: usize) -> usize {
    println!("{:04} ", offset);

    match c[offset] {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

#[cfg(test)]
mod byte_parser_test {
    use super::*;

    #[test]
    fn basic() {
        let mut progm = Chunk::new();
        progm.push(OpCode::OpReturn);

        disassemble_chunk(progm, "test");
    }
}
