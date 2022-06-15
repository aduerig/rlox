use std::io::Write;

type Value = f64;


#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
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


#[allow(dead_code)]
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

#[allow(dead_code)]
fn disassemble_chunk(chunk: Chunk) {
    println!("=== chunk ===");

    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        offset += disassemble_instruction(&chunk, offset);
    }
}



// vm stuff
#[derive(PartialEq)]
enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}


#[allow(dead_code)]
struct VirtualMachine {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}


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

#[allow(dead_code)]
struct Scanner {
    source: String,
    current: usize,
    line: i64,
}

#[allow(dead_code)]
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
    Comment, 
    Eof,
}

#[allow(dead_code)]
struct Token {
    token_type: TokenType,
    data: String,
    line: i64,
}

fn make_token(token_type: TokenType, source: &String, index: &mut usize, size: usize, line: i64) -> Token {
    let something = Token {
        token_type: token_type,
        data: (&source[*index..(*index+size)]).to_string(),
        line: line
    };
    *index = *index + size;
    return something;
}


fn check_next_token(source: &String, index: usize, token_size: &mut usize, the_char: char) -> bool {
    if index + 1 >= source.len() {
        return false;
    }
    if source.chars().nth(index + 1).unwrap() == the_char {
        *token_size += 1;
        return true;
    }
    return false;
}

fn scan_token(source: &String, index: &mut usize, lines: &mut i64) -> Token {
    // if *index as usize >= source.len() {
    //     return Token {
    //         token_type: TokenType::Eof, 
    //         data: "".to_string(), 
    //         line: *lines,
    //     };
    // }

    let mut token_size = 1usize;
    let the_char = source.chars().nth(*index).unwrap();
    match the_char {
        '(' => make_token(TokenType::LeftParen, source, index, token_size, *lines),
        ')' => make_token(TokenType::RightParen, source, index, token_size, *lines),
        '{' => make_token(TokenType::LeftBrace, source, index, token_size, *lines),
        '}' => make_token(TokenType::RightBrace, source, index, token_size, *lines),
        ';' => make_token(TokenType::Semicolon, source, index, token_size, *lines),
        ',' => make_token(TokenType::Comma, source, index, token_size, *lines),
        '.' => make_token(TokenType::Dot, source, index, token_size, *lines),
        '-' => make_token(TokenType::Minus, source, index, token_size, *lines),
        '+' => make_token(TokenType::Plus, source, index, token_size, *lines),
        '*' => make_token(TokenType::Star, source, index, token_size, *lines),
        '!' => make_token(if check_next_token(source, *index, &mut token_size, '=') { TokenType::BangEqual } else { TokenType::Bang }, &source, index, token_size, *lines),
        '=' => make_token(if check_next_token(source, *index, &mut token_size, '=') { TokenType::EqualEqual } else { TokenType::Equal }, &source, index, token_size, *lines),
        '<' => make_token(if check_next_token(source, *index, &mut token_size, '=') { TokenType::LessEqual } else { TokenType::Less }, &source, index, token_size, *lines),
        '>' => make_token(if check_next_token(source, *index, &mut token_size, '=') { TokenType::GreaterEqual } else { TokenType::Greater }, &source, index, token_size, *lines),
        '/' => {
            if !check_next_token(source, *index, &mut token_size, '/') {
                return make_token(TokenType::Slash, source, index, token_size, *lines);
            }
            while *index + token_size < source.len() && source.chars().nth(*index).unwrap() != '\n' {
                token_size += 1;
            }
            return make_token(TokenType::Comment, source, index, token_size, *lines);
        },
        _ => { 
                *index += 1; 
                return Token {
                    token_type: TokenType::Error, 
                    data: "Unexpected character".to_string(), 
                    line: *lines,
                }
            }
    }
}


fn advance_to_next_token_index(source: &String, index: &mut usize, lines: &mut i64) -> bool {
    while *index < source.len() {
        let the_char = source.chars().nth(*index).unwrap();
        if the_char == '\n' {
            *index += 1;
            *lines += 1; 
        }
        else if the_char == ' ' {
            *index += 1;
        }
        else if the_char == '\t' {
            *index += 1;
        }
        else if the_char == '\r' {
            *index += 1;
        }
        else {
            break
        }
    }
    return *index < source.len();
}

fn compile(source: &String) {
    println!("Starting compilation of source code\n{}", source);
    let mut index = 0usize;
    let mut lines = 1;
    while advance_to_next_token_index(source, &mut index, &mut lines) {
        let old_index = index;
        let _token: Token = scan_token(&source, &mut index, &mut lines);
        println!("index: {}, data: {}, tokentype: {}", old_index, _token.data, _token.token_type as u8);
        // if (token.line != line) {
        //     printf("%4d ", token.line);
        //     line = token.line;
        // } else {
        //     printf(" | ");
        // }
        // printf("%2d '%.*s'\n", token.token_type, token.length, token.start);
        // if (token.type == TOKEN_EOF) break;
    }
    println!("Finished compilation of source code:\n{}", source);
}


fn interpret(source: String) -> InterpretResult {
    compile(&source);
    // return run();
    return InterpretResult::InterpretOk;
}

fn repl() {
    print!("> ");
    let _result = std::io::stdout().flush();
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    interpret(line);
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
