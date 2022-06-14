type Value = f64;


#[repr(u8)]
#[derive(Debug)]
enum OpCode {
    Return,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Constant,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


struct Chunk {
    code: Vec<u8>,
    lines: Vec<i64>,
    constants: Vec<Value>,
}



fn add_constant(chunk: &mut Chunk, value: Value, line: i64) {
    chunk.code.push(OpCode::Constant as u8);
    chunk.constants.push(value);
    chunk.code.push(chunk.constants.len() as u8 - 1);
    chunk.lines.push(line);
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("Disassembling - Instruction at offset {}", offset);
    if chunk.lines.len() > offset && offset > 0 && chunk.lines[offset] == chunk.lines[offset-1] {
        print!("  | ");
    }
    else if chunk.lines.len() > offset {
        print!("{: >4}", chunk.lines[offset]);
    }

    
    let instruction = chunk.code[offset];
    if instruction == OpCode::Return as u8 {
        // main book creates simpleInstruction() here
        println!(" === OpCode::Return");
        return 1;
    }
    else if instruction == OpCode::Constant as u8 {
        let constant_index = chunk.code[offset + 1];
        let value = chunk.constants[constant_index as usize];
        println!(": OpCode::Constant = {}", value);
        return 2;
    }
    else {
        println!("UNKNOWN");
        return 1;
    }
}


fn disassemble_chunk(chunk: Chunk) {
    println!("=== chunk ===");

    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        offset += disassemble_instruction(&chunk, offset);
    }
}



// vm stuff
enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}


struct VirtualMachine {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}


// #define READ_BYTE() (*vm.ip++)
fn run(vm: &mut VirtualMachine) -> InterpretResult {
    while vm.ip < vm.chunk.code.len() {
        let instruction = vm.chunk.code[vm.ip];
        disassemble_instruction(&vm.chunk, vm.ip);
        
        // println!("{}");

        vm.ip += 1;
        if instruction == OpCode::Return as u8 {
            println!("Returning and popping value: {}", vm.stack.pop().unwrap());
        }
        else if instruction == OpCode::Constant as u8 {
            let constant_index = vm.chunk.code[vm.ip];
            let constant = vm.chunk.constants[constant_index as usize];
            vm.stack.push(constant);
            vm.ip += 1;
        }
        else if instruction == OpCode::Negate as u8 {
            let stack_val = vm.stack.pop().unwrap();
            vm.stack.push(-stack_val);
        }
        else if instruction == OpCode::Add as u8 {
            let stack_val2 = vm.stack.pop().unwrap();
            let stack_val1 = vm.stack.pop().unwrap();
            vm.stack.push(stack_val1 + stack_val2);
        }
        else if instruction == OpCode::Multiply as u8 {
            let stack_val2 = vm.stack.pop().unwrap();
            let stack_val1 = vm.stack.pop().unwrap();
            vm.stack.push(stack_val1 * stack_val2);
        }
        else if instruction == OpCode::Subtract as u8 {
            let stack_val2 = vm.stack.pop().unwrap();
            let stack_val1 = vm.stack.pop().unwrap();
            vm.stack.push(stack_val1 - stack_val2);
        }
        else if instruction == OpCode::Divide as u8 {
            let stack_val2 = vm.stack.pop().unwrap();
            let stack_val1 = vm.stack.pop().unwrap();
            vm.stack.push(stack_val1 / stack_val2);
        }
    }
    return InterpretResult::InterpretOk;
}






// let mut chunk = Chunk{
//     code: Vec::new(),
//     lines: Vec::new(),
//     constants: Vec::new(),
// };

// add_constant(&mut chunk, 1.2f64, 148);
// add_constant(&mut chunk, 3.4f64, 148);
// chunk.code.push(OpCode::Add as u8);
// chunk.lines.push(148);


// add_constant(&mut chunk, 5.6f64, 148);
// chunk.code.push(OpCode::Divide as u8);
// chunk.lines.push(148);

// chunk.code.push(OpCode::Negate as u8);
// chunk.lines.push(148);

// chunk.code.push(OpCode::Return as u8);
// chunk.lines.push(148);


// // disassemble_chunk(chunk);

// let mut vm = VirtualMachine {
//     chunk: chunk,
//     ip: 0,
//     stack: Vec::new(),
// };

// run(&mut vm);


struct Scanner {
    source: String,
    current: usize,
    line: i64,
}


enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, 
    LeftBrace, RightBrace, 
    Comma, Dot, Minus, Plus, 
    Semicolon, Slash, Star, 
    // One or two character tokens.
    Bang, BangEqual, 
    Equal, EqualEqual, 
    Greater, GreaterEqual, 
    Less, LessEqual, 
    // Literals.
    Identifier, String, Number, 
    // Keywords.
    And, Class, Else, False, 
    For, Fun, If, Nil, Or, 
    Print, Return, Super, This, 
    True, Var, While, 
    Error, 
    Eof,
}

struct Token {
    token_type: TokenType,
    data: String,
    line: i64,
}

fn make_token(token_type: TokenType) -> Token {
    return Token {
        token_type: token_type, 
        data: "idk".to_string(), 
        line: 0
    };
}

fn scan_token() -> Token {
    // return make_token(TokenType::Eof);
    
    return Token {token_type: TokenType::Error, data: "Unexpected character.".to_string(), line: 0};
}


fn compile(source: &String) {
    loop {
        Token token = scan_token();
        if (token.line != line) {
        printf("%4d ", token.line);
        line = token.line;
        } else {
        printf(" | ");
        }
        printf("%2d '%.*s'\n", token.type, token.length, token.start);
        if (token.type == TOKEN_EOF) break;
    }
}


fn interpret(source: &String) -> InterpretResult {
    compile(source);
    // return run();
    return InterpretResult::InterpretOk;
}

fn repl() {
    print!("> ");
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    interpret(b1);
}


fn read_file_to_string(filepath: &String) -> String {
    return std::fs::read_to_string(filepath).unwrap();
}

fn run_file(filepath: &String) {
    let source = read_file_to_string(&filepath);
    let result = interpret(source);
    if result == InterpretResult::InterpretCompileError {
        std::process::exit(65);
    }
    if result == InterpretResult::InterpretRuntimeError {
        std::process::exit(70);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    if args.len() == 1 {
        println!("=== Starting REPL ===");
        return repl();
    }
    run_file(&args[1]);
}
