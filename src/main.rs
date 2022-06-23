use std::io::Write;
use enum_map::{enum_map, Enum};
use colored::Colorize;

type Value = f64;


#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ValueType {
    Bool(bool),
    None,
    Number(f64),
}
impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


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

// impl std::fmt::Display for OpCode {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }


#[derive(Default)]
struct Chunk {
    code: Vec<u8>,
    lines: Vec<i64>,
    constants: Vec<Value>,
}


#[allow(dead_code)]
fn add_constant(chunk: &mut Chunk, value: Value, line: i64) {
    if chunk.code.len() as u8 >= std::u8::MAX {
        println!("Too many constants in the pool!");
        panic!();
    }
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
        println!(": OpCode::Return");
        return 1;
    }
    else if instruction == OpCode::Add as u8 {
        println!(": OpCode::Add");
        return 1;
    }
    else if instruction == OpCode::Subtract as u8 {
        println!(": OpCode::Subtract");
        return 1;
    }
    else if instruction == OpCode::Divide as u8 {
        println!(": OpCode::Divide");
        return 1;
    }
    else if instruction == OpCode::Multiply as u8 {
        println!(": OpCode::Multiply");
        return 1;
    }
    else if instruction == OpCode::Constant as u8 {
        let constant_index = chunk.code[offset + 1];
        let value = chunk.constants[constant_index as usize];
        println!(": OpCode::Constant = {}", value);
        return 2;
    }
    else {
        println!(": UNKNOWN: {}", instruction);
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
#[derive(PartialEq, Debug)]
enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
impl std::fmt::Display for InterpretResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn run(vm: &mut VirtualMachine) -> InterpretResult {
    println!("=== NOW RUNNING ===");
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
    return InterpretResult::Ok;
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

#[derive(PartialEq, Clone, Debug, Enum, Copy)]
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
    For, Fun, If, Null, Or, 
    Print, Return, Super, This, 
    True, Var, While, 
    Error, 
    Comment, 
    // Eof,
}
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
#[derive(PartialEq)]
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

fn is_digit(source: &String, index: usize) -> bool {
    return source.chars().nth(index).unwrap().is_digit(10);
}

fn scan_text_and_make_tokens(source: &String, index: &mut usize, lines: &mut i64) -> Token {
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
            println!("finished the comment parse at {}", *index + 1);
            return make_token(TokenType::Comment, source, index, token_size, *lines);
        },
        '"' => {
            while *index + token_size < source.len() && source.chars().nth(*index + token_size).unwrap() != '"' {
                if source.chars().nth(*index).unwrap() == '\n' {
                    *lines += 1;
                }
                token_size += 1;
            }
            if *index + token_size >= source.len() {
                *index += token_size;
                return Token {
                    token_type: TokenType::Error, 
                    data: "TOKEN ERROR: No end quote".to_string(), 
                    line: *lines,
                }
            }
            return make_token(TokenType::String, source, index, token_size + 1, *lines);
        },
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
            while *index + token_size < source.len() {
                let next_char = source.chars().nth(*index + token_size).unwrap();
                if next_char.is_digit(10) || next_char == '.' {
                    token_size += 1;
                }
                else {
                    break;
                }
            }
            return make_token(TokenType::Number, source, index, token_size, *lines);
        },
        'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z'|'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'L'|'M'|'N'|'O'|'P'|'Q'|'R'|'S'|'T'|'U'|'V'|'W'|'X'|'Y'|'Z'|'_' => {
            while *index + token_size < source.len() {
                let next_char = source.chars().nth(*index + token_size).unwrap();
                if next_char.is_alphanumeric() || next_char == '_' {
                    token_size += 1;
                }
                else {
                    break;
                }
            }
            let token_type = match &source[*index..(*index+token_size)] {
                "and" => TokenType::And,
                "class" => TokenType::Class,
                "else" => TokenType::Else,
                "if" => TokenType::If,
                "null" => TokenType::Null,
                "or" => TokenType::Or,
                "print" => TokenType::Print,
                "return" => TokenType::Return,
                "super" => TokenType::Super,
                "var" => TokenType::Var,
                "while" => TokenType::While,
                "true" => TokenType::True,
                "false" => TokenType::False,
                "for" => TokenType::For,
                "fun" => TokenType::Fun,
                "this" => TokenType::This,
                _ => TokenType::Identifier
            };

            return make_token(token_type, source, index, token_size, *lines);
        }
        _ => { 
                *index += 1; 
                return Token {
                    token_type: TokenType::Error, 
                    data: "TOKEN ERROR: Unexpected character".to_string(), 
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


// fn scan(source: &String) -> (bool, Vec<Token>) {
//     println!("Starting compilation of source code\n{}", source);
//     let mut index = 0usize;
//     let mut lines = 1;
//     let mut succeeded = true;
//     let mut all_tokens = vec!();
//     while advance_to_next_token_index(source, &mut index, &mut lines) {
//         let old_index = index;
//         let token: Token = scan_text_and_make_tokens(&source, &mut index, &mut lines);

//         // why tf does this MOVE the thing?
//         // println!("index: {}, data: {}, tokentype: {}", old_index, token.data, token.token_type as u8);
//         println!("index: {}, data: {}, tokentype: {}", old_index, token.data, token.token_type.clone() as u8);
//         if token.token_type == TokenType::Error {
//             succeeded = false;
//         }
//         all_tokens.push(token);
//         // if (token.line != line) {
//         //     printf("%4d ", token.line);
//         //     line = token.line;
//         // } else {
//         //     printf(" | ");
//         // }
//         // printf("%2d '%.*s'\n", token.token_type, token.length, token.start);
//         // if (token.type == TOKEN_EOF) break;
//     }
//     println!("Finished compilation of source code:\n{}", source);
//     return (succeeded, all_tokens);
// }


struct VirtualMachine {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}





fn scan(source: &String) -> (bool, Vec<Token>) {
    println!("=== Starting scanning of source code ===\n{}", source);
    let mut index = 0usize;
    let mut lines = 1;
    let mut succeeded = true;
    let mut all_tokens = vec!();
    while advance_to_next_token_index(source, &mut index, &mut lines) {
        let old_index = index;
        let token: Token = scan_text_and_make_tokens(&source, &mut index, &mut lines);

        println!("index: {}, data: {}, tokentype: {}", old_index, token.data, token.token_type.clone() as u8);
        if token.token_type == TokenType::Error {
            succeeded = false;
        }
        all_tokens.push(token);
    }
    println!("Finished scanning of source code");
    return (succeeded, all_tokens);
}


fn emit_byte(chunk: &mut Chunk, byte: u8) {
    chunk.code.push(byte);
}


// add_constant(chunk, value, 0))
// '3.14'.parse().unwrap()


#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Enum, Copy, Clone)]
enum Precedence {
    None, 
    Assignment, // =
    Or, // Or
    And, // And
    Equality, // ==!=
    Comparison, // <><=>=
    Term, // +-
    Factor, // */
    Unary, // !-
    Call, // .()
    Primary,
}

impl std::fmt::Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


fn next_prec(precedence: Precedence) -> Precedence {
    let next_prec_map = enum_map! {
        Precedence::None => Precedence::Assignment,
        Precedence::Assignment => Precedence::Or,
        Precedence::Or => Precedence::And,
        Precedence::And => Precedence::Equality,
        Precedence::Equality => Precedence::Comparison,
        Precedence::Comparison => Precedence::Term,
        Precedence::Term => Precedence::Factor,
        Precedence::Factor => Precedence::Unary,
        Precedence::Unary => Precedence::Call,
        Precedence::Call => Precedence::Primary,
        Precedence::Primary => Precedence::Primary,
    };
    return next_prec_map[precedence];
}


#[derive(Copy, Clone)]
struct ParseRule {
    prefix: Option<fn(&mut Chunk, &Vec<Token>, &mut usize)>,
    infix: Option<fn(&mut Chunk, &Vec<Token>, &mut usize)>,
    precedence: Precedence
}

fn get_rule(token_type: TokenType) -> ParseRule {
    let rules = enum_map! {
        TokenType::LeftParen => ParseRule {prefix: Some(grouping), infix: None, precedence: Precedence::None}, 
        TokenType::RightParen => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::LeftBrace => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::RightBrace => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Comma => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Dot => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Minus => ParseRule {prefix: Some(unary), infix: Some(binary), precedence: Precedence::Term}, 
        TokenType::Plus => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Term}, 
        TokenType::Semicolon => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Slash => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Factor}, 
        TokenType::Star => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Factor}, 
        TokenType::Bang => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::BangEqual => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Equal => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::EqualEqual => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Greater => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::GreaterEqual => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Less => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::LessEqual => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Identifier => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::String => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        // FIX THIS BELOW, SHOULD BE NUMBER, NOT BINARY
        TokenType::Number => ParseRule {prefix: Some(number), infix: None, precedence: Precedence::None}, 
        
        TokenType::And => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Class => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Else => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::False => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::For => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Fun => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::If => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Null => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Or => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Print => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Return => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Super => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::This => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::True => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Var => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::While => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Error => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 

        // THIS IS JUNK
        TokenType::Comment => ParseRule {prefix: Some(binary), infix: None, precedence: Precedence::None}, 
    };
    return rules[token_type];
}

fn parse_precedence(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize, precedence: Precedence) {
    println!("PRECEDENCE: {}, index: {}, precedence value of {}", precedence.to_string(), *index, precedence as u8);
    *index += 1;
    let prefix_function = get_rule(all_tokens[*index-1].token_type).prefix;
    match prefix_function {
        Some(x) => x(chunk, all_tokens, index),
        None => println!("Prefix, expected expression!"),
    };

    while *index < all_tokens.len() {   
        if precedence as u8 <= get_rule(all_tokens[*index].token_type).precedence as u8 {
            let parse_rule = get_rule(all_tokens[*index].token_type);
            println!("infix time! token: {}, token_type: {}, index: {}, parse_rule here is {}", all_tokens[*index].data, all_tokens[*index].token_type.to_string(), *index, parse_rule.precedence);
            *index += 1;
            match parse_rule.infix {
                Some(x) => x(chunk, all_tokens, index),
                None => (),
            };
        }
        else {
            println!("No proper infix at index: {}, precedence is {}", *index, get_rule(all_tokens[*index].token_type).precedence.to_string());
            return;
        }
    }
    println!("Index went out of range!");
}

fn number(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    let str_data = &all_tokens[*index-1].data;
    let value: Value = str_data.parse::<f64>().unwrap();
    println!("Running number at index {}, number is: {}", *index-1, value);
    add_constant(chunk, value, 0);
}

fn binary(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    println!("Running binary at index {}", *index);
    let last_token_type: TokenType = all_tokens[*index - 1].token_type;
    
    let rule: ParseRule = get_rule(last_token_type);
    parse_precedence(chunk, all_tokens, index, next_prec(rule.precedence));

    match last_token_type {
        TokenType::Plus => emit_byte(chunk, OpCode::Add as u8),
        TokenType::Minus => emit_byte(chunk, OpCode::Subtract as u8),
        TokenType::Star => emit_byte(chunk, OpCode::Multiply as u8),
        TokenType::Slash => emit_byte(chunk, OpCode::Divide as u8),
        _ => println!("Not implemented lol"),
    }
}

fn unary(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    println!("Running unary at index {}", *index);
    // expression(chunk, all_tokens, index);
    parse_precedence(chunk, all_tokens, index, Precedence::Unary);
    emit_byte(chunk, OpCode::Negate as u8);
}

fn grouping(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    println!("Running grouping at index {}", *index);
    expression(chunk, all_tokens, index);
    consume(all_tokens, index, TokenType::RightParen, "Expected a right parenthesis to end the group");
}


fn consume(all_tokens: &Vec<Token>, index: &mut usize, expected_token: TokenType, error_message: &str) -> bool {
    if all_tokens[*index].token_type == expected_token {
        *index += 1;
        return true;
    }
    let lmao = format!("ERROR! Expected to consume tokentype \"{}\" at index: {}, but got {}", expected_token.to_string(), *index, all_tokens[*index].data);
    println!("{}", lmao.red());
    println!("{}", error_message);
    return false;
}



fn expression(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    parse_precedence(chunk, all_tokens, index, Precedence::Assignment)
}


fn compile(source: &String) -> (bool, Chunk) {
    let (success, all_tokens) = scan(&source);
    if !success {
        println!("Scanner failed parsing a token somewhere");
        return (false, Default::default())
    }

    let mut chunk = Chunk {
        code: vec!(),
        lines: vec!(),
        constants: vec!(),        
    };

    println!("=== Starting compile ===");
    expression(&mut chunk, &all_tokens, &mut 0);
    emit_byte(&mut chunk, OpCode::Return as u8);
    return (true, chunk);
}


fn interpret(source: String) -> InterpretResult {
    let (_success, chunk) = compile(&source);
    return run(&mut VirtualMachine {
        chunk: chunk,
        ip: 0,
        stack: vec!(),        
    });
}

fn full_lines(mut input: impl std::io::BufRead) -> impl Iterator<Item = String> {
    std::iter::from_fn(move || {
        let mut vec = String::new();
        let string_length = input.read_line(&mut vec);
        if string_length.unwrap() == 1 {
            return None;
        }
        return Some(vec);
    })
}

fn repl() {
    print!("> ");
    let _result = std::io::stdout().flush();
    let mut source = String::new();
    for line in full_lines(std::io::stdin().lock()) {
        // Now print the line (line.unwrap() first) via the println!() macro
        source += &line;
    }
    // let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    interpret(source);
}


fn read_file_to_string(filepath: &String) -> String {
    println!("reading from filepath: {}", filepath);
    return std::fs::read_to_string(filepath).unwrap();
}

fn run_file(filepath: &String) {
    let source = read_file_to_string(&filepath);
    let result = interpret(source);
    if result == InterpretResult::CompileError {
        std::process::exit(65);
    }
    if result == InterpretResult::RuntimeError {
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
