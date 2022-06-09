#[repr(u8)]
enum OpCode {
    Return,
    Constant,
}

struct Chunk {
    code: Vec<u8>,
}


fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    if chunk.code[offset] == OpCode::Return as u8 {
        println!("Return instruction");
        1
    }
    else if chunk.code[offset] == OpCode::Constant as u8 {
        println!("Constant instruction");
        2
    }
    else {
        println!("Unknown instruction");
        1
    }
}


fn disassemble_chunk(chunk: Chunk) {
    println!("=== chunk ===");

    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        let instruction = disassemble_instruction(&chunk, offset)
        offset += instruction.;
    }
}


fn main() {
    println!("Hello, world!");

    let mut code: Vec<u8> = Vec::new();
    code.push(OpCode::Return as u8);
    code.push(OpCode::Constant as u8);
    code.push(2u8);

    let chunk = Chunk{code: code};
    disassemble_chunk(chunk);
}
