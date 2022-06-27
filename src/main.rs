use std::io::Write;
use std::iter::zip;
use std::collections::HashMap;
use enum_map::{enum_map, Enum};
use colored::Colorize;

// type Value = f64;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ObjData {
    String(Vec<char>),
}

#[repr(u8)]
#[derive(Clone, PartialEq)]
#[allow(dead_code)]
enum Value {
    Bool(bool),
    Null,
    Number(f64),
    Obj(ObjData),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", get_value_str_with_quotes(self))
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
    Null,
    True,
    False,
    Equal,
    Greater,
    Less,
    Not,
    Print,
    JumpIfFalse,
    Pop,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
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
        panic!("Too many constants in the pool!");
    }
    chunk.code.push(OpCode::Constant as u8);
    chunk.constants.push(value);
    chunk.code.push(chunk.constants.len() as u8 - 1);
    chunk.lines.push(line);
}

fn add_constant_dont_emit(chunk: &mut Chunk, value: Value, line: i64) -> u8 {
    if chunk.code.len() as u8 >= std::u8::MAX {
        panic!("Too many constants in the pool!");
    }
    chunk.constants.push(value);
    chunk.lines.push(line);
    return chunk.constants.len() as u8 - 1;
}

fn get_value_str(value: &Value) -> String {    
    match value {
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Number(num) => num.to_string(),
        Value::Obj(obj1) => {
            let mut the_string = "".to_string();
            if let ObjData::String(char_vec) = obj1 {
                for i in char_vec {
                    the_string += &i.to_string();
                }
                return the_string;
            }
            panic!("Yeah idk string stuff");
        },
    }        
}

fn get_value_str_with_quotes(value: &Value) -> String {    
    match value {
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Number(num) => num.to_string(),
        Value::Obj(obj1) => {
            let mut the_string = "\"".to_string();
            if let ObjData::String(char_vec) = obj1 {
                for i in char_vec {
                    the_string += &i.to_string();
                }
                the_string += "\"";
                return the_string;
            }
            panic!("Yeah idk string stuff");
        },
    }        
}

fn disassemble_and_print_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("Disassembling - Instruction at offset {}", offset);
    if chunk.lines.len() > offset && offset > 0 && chunk.lines[offset] == chunk.lines[offset-1] {
        print!("  | ");
    }
    else if chunk.lines.len() > offset {
        print!("{: >4}", chunk.lines[offset]);
    }

    
    let instruction = chunk.code[offset];
    if instruction == OpCode::Return as u8 {
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
    else if instruction == OpCode::True as u8 {
        println!(": OpCode::True");
        return 1;
    }
    else if instruction == OpCode::False as u8 {
        println!(": OpCode::False");
        return 1;
    }
    else if instruction == OpCode::Null as u8 {
        println!(": OpCode::Null");
        return 1;
    }
    else if instruction == OpCode::Multiply as u8 {
        println!(": OpCode::Multiply");
        return 1;
    }
    else if instruction == OpCode::Not as u8 {
        println!(": OpCode::Not");
        return 1;
    }
    else if instruction == OpCode::Equal as u8 {
        println!(": OpCode::Equal");
        return 1;
    }
    else if instruction == OpCode::Less as u8 {
        println!(": OpCode::Less");
        return 1;
    }
    else if instruction == OpCode::Greater as u8 {
        println!(": OpCode::Greater");
        return 1;
    }
    else if instruction == OpCode::Negate as u8 {
        println!(": OpCode::Negate");
        return 1;
    }
    else if instruction == OpCode::Pop as u8 {
        println!(": OpCode::Pop");
        return 1;
    }
    else if instruction == OpCode::DefineGlobal as u8 {
        println!(": OpCode::DefineGlobal");
        return 1;
    }
    else if instruction == OpCode::GetGlobal as u8 {
        println!(": OpCode::GetGlobal");
        return 1;
    }
    else if instruction == OpCode::SetGlobal as u8 {
        println!(": OpCode::SetGlobal");
        return 1;
    }
    else if instruction == OpCode::Constant as u8 {
        let constant_index = chunk.code[offset + 1];
        let value = &chunk.constants[constant_index as usize];
        println!(": OpCode::Constant = {}", get_value_str_with_quotes(&value));
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
        offset += disassemble_and_print_instruction(&chunk, offset);
    }
}



// vm stuff
#[derive(PartialEq, Debug)]
enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

fn values_equal(val1: Value, val2: Value) -> bool {
    if let Value::Null = val1 {
        if let Value::Null = val2 {
            return true;
        }
    }
    if let Value::Number(num1) = val1 {
        if let Value::Number(num2) = val2 {
            return num1 == num2;
        }
    }
    if let Value::Bool(b1) = val1 {
        if let Value::Bool(b2) = val2 {
            return b1 == b2;
        }
    }

    // Can do string interning here (page 370) for perf increase, map strings to value to compare
    if let Value::Obj(obj1) = val1 {
        if let ObjData::String(char_vec1) = obj1 {
            if let Value::Obj(obj2) = val2 {
                if let ObjData::String(char_vec2) = obj2 {
                    if char_vec1.len() == char_vec2.len() {
                        for (a, b) in zip(char_vec1, char_vec2) {
                            if a != b {
                                return false;
                            }
                        }
                        return true;
                    }
                }
            }        
        }
    }
    return false;
}

fn values_greater(val1: Value, val2: Value) -> bool {
    if let Value::Number(num1) = val1 {
        if let Value::Number(num2) = val2 {
            return num1 > num2;
        }
    }
    if let Value::Bool(b1) = val1 {
        if let Value::Bool(b2) = val2 {
            return b1 > b2;
        }
    }
    if let Value::Obj(_) = val1 {
        panic!("Cant value greater an object, sorry bud")
    }
    if let Value::Obj(_) = val2 {
        panic!("Cant value greater an object, sorry bud")
    }
    return false;
}


fn values_less(val1: Value, val2: Value) -> bool {
    if let Value::Number(num1) = val1 {
        if let Value::Number(num2) = val2 {
            return num1 < num2;
        }
    }
    if let Value::Bool(b1) = val1 {
        if let Value::Bool(b2) = val2 {
            return b1 < b2;
        }
    }
    if let Value::Obj(_) = val1 {
        panic!("Cant value less an object, sorry bud")
    }
    if let Value::Obj(_) = val2 {
        panic!("Cant value less an object, sorry bud")
    }
    return false;
}


fn run(vm: &mut VirtualMachine) -> InterpretResult {
    println!("=== NOW RUNNING ===");

    println!("{:?}", vm.chunk.code);
    while vm.ip < vm.chunk.code.len() {
        println!("Execution: {}, Current state of stack: {:?}", vm.ip, vm.stack);

        let instruction = vm.chunk.code[vm.ip];
        disassemble_and_print_instruction(&vm.chunk, vm.ip);

        vm.ip += 1;
        if instruction == OpCode::Return as u8 {
            println!("Return found. Printing and popping value: {:?}", vm.stack.pop().unwrap());
            continue;
        }
        else if instruction == OpCode::Pop as u8 {
            println!("Pop found: {:?}", vm.stack.pop().unwrap());
            continue;
        }
        else if instruction == OpCode::DefineGlobal as u8 {
            println!("DefineGlobal found");
            let variable_equal_to = vm.stack.pop().unwrap();

            let constant_index = vm.chunk.code[vm.ip];
            println!("the byte is: {}", constant_index);
            // disassemble_and_print_instruction(&vm.chunk, vm.ip);

            let constant = &vm.chunk.constants[constant_index as usize];


            if let Value::Obj(obj) = constant {
                if let ObjData::String(string) = obj {
                    let mut new_string = "".to_string();
                    for i in string {
                        new_string += &i.to_string();
                    }
                    vm.globals.insert(new_string, variable_equal_to);
                    vm.ip += 1;
                    continue;
                }
            }
            panic!("DefiningGlobal must have a string constant after it");
        }
        else if instruction == OpCode::GetGlobal as u8 {
            println!("GetGlobal found");
            let constant_index = vm.chunk.code[vm.ip];
            let constant = &vm.chunk.constants[constant_index as usize];                
            vm.ip += 1;

            if let Value::Obj(obj) = constant {
                if let ObjData::String(string) = obj {
                    let mut new_string = "".to_string();
                    for i in string {
                        new_string += &i.to_string();
                    }
                    let result = vm.globals.get(&new_string);
                    match result {
                        Some(x) => vm.stack.push(x.clone()),
                        None => panic!("Tried to access a variable that doesn't exist"),
                    }
                    continue;
                }
            }
            panic!("DefiningGlobal must have a string constant after it");
        }
        else if instruction == OpCode::SetGlobal as u8 {
            println!("SetGlobal found");
            let variable_equal_to = vm.stack.pop().unwrap();

            let constant_index = vm.chunk.code[vm.ip];
            println!("the byte is: {}", constant_index);
            // disassemble_and_print_instruction(&vm.chunk, vm.ip);

            let constant = &vm.chunk.constants[constant_index as usize];


            if let Value::Obj(obj) = constant {
                if let ObjData::String(string) = obj {
                    let mut new_string = "".to_string();
                    for i in string {
                        new_string += &i.to_string();
                    }
                    vm.globals.insert(new_string, variable_equal_to.clone());
                    vm.ip += 1;
                    vm.stack.push(variable_equal_to);
                    continue;
                }
            }
            panic!("SetGlobal must have a string constant after it");
        }
        else if instruction == OpCode::Constant as u8 {
            let constant_index = vm.chunk.code[vm.ip];
            let constant = &vm.chunk.constants[constant_index as usize];
            vm.stack.push(constant.clone());
            vm.ip += 1;
            continue;
        }
        else if instruction == OpCode::True as u8 {
            vm.stack.push(Value::Bool(true));
            continue;
        }
        else if instruction == OpCode::False as u8 {
            vm.stack.push(Value::Bool(false));
            continue;
        }
        else if instruction == OpCode::Null as u8 {
            vm.stack.push(Value::Null);
            continue;
        }
        else if instruction == OpCode::Print as u8 {
            println!("Printing: {}", get_value_str(&vm.stack.pop().unwrap()));
            continue;
        }
        else if instruction == OpCode::JumpIfFalse as u8 {
            println!("if statement");
            let short_part_1 = (vm.chunk.code[vm.ip] as usize) << 8;
            let short_part_2 = vm.chunk.code[vm.ip + 1] as usize;
            let jump_forward = short_part_1 + short_part_2;
            vm.ip += 2;

            let result_of_if = match vm.stack.pop().unwrap() {
                Value::Null => false,
                Value::Bool(b) => b,
                Value::Number(_) => true,
                Value::Obj(_) => true,
                // _ => panic!("You can't negate this, idk even what it is"),
            };

            if !result_of_if {
                println!("False branch, jumping!");
                vm.ip += jump_forward;
            }
            continue;
        }
        else if instruction == OpCode::Not as u8 {
            match vm.stack.pop().unwrap() {
                Value::Null => panic!("You can't not a null!"),
                Value::Bool(b) => vm.stack.push(Value::Bool(!b)),
                Value::Number(_) => panic!("You can't not a number!"),
                Value::Obj(_) => panic!("You can't not an obj!"),
                // _ => panic!("You can't negate this, idk even what it is"),
            }
            continue;
        }
        else if instruction == OpCode::Negate as u8 {
            match vm.stack.pop().unwrap() {
                Value::Null => panic!("You can't negate a null!"),
                Value::Bool(b) => vm.stack.push(Value::Bool(!b)),
                Value::Number(num) => vm.stack.push(Value::Number(-num)),
                Value::Obj(_) => panic!("You can't negate an object!"),
                // _ => panic!("You can't negate this, idk even what it is"),
            }
            continue;
        }
        else if instruction == OpCode::Equal as u8 {
            let stack_val1 = vm.stack.pop().unwrap();
            let stack_val2 = vm.stack.pop().unwrap();
            vm.stack.push(Value::Bool(values_equal(stack_val2, stack_val1)));
            continue;
        }
        else if instruction == OpCode::Less as u8 {
            let stack_val1 = vm.stack.pop().unwrap();
            let stack_val2 = vm.stack.pop().unwrap();
            vm.stack.push(Value::Bool(values_less(stack_val2, stack_val1)));
            continue;
        }
        else if instruction == OpCode::Greater as u8 {
            let stack_val1 = vm.stack.pop().unwrap();
            let stack_val2 = vm.stack.pop().unwrap();
            vm.stack.push(Value::Bool(values_greater(stack_val2, stack_val1)));
            continue;
        }
        // Assuming its a binary operation

        let stack_val1 = vm.stack.pop().unwrap();
        let stack_val2 = vm.stack.pop().unwrap();

        if let Value::Obj(obj1) = stack_val1 {
            if let ObjData::String(right_char_vec) = obj1 {
                if let Value::Obj(obj2) = stack_val2 {
                    if let ObjData::String(left_char_vec) = obj2 {
                        // Does this leak memory? The new dynamically allocatd string that ets pushed on the stack
                        if instruction == OpCode::Add as u8 {
                            let mut new_char_vec = left_char_vec.clone();
                            for i in right_char_vec {
                                new_char_vec.push(i);
                            }
                            let new_string = Value::Obj(ObjData::String(new_char_vec));
                            vm.stack.push(new_string);
                            continue;
                        }
                        panic!("Can't do this operation on a string!");
                    }
                }        
            }
        }
        else if let Value::Number(right_val) = stack_val1 {
            if let Value::Number(left_val) = stack_val2 {
                if instruction == OpCode::Add as u8 {
                    vm.stack.push(Value::Number(left_val + right_val));
                }
                else if instruction == OpCode::Multiply as u8 {
                    vm.stack.push(Value::Number(left_val * right_val));
                }
                else if instruction == OpCode::Subtract as u8 {
                    vm.stack.push(Value::Number(left_val - right_val));
                }
                else if instruction == OpCode::Divide as u8 {
                    vm.stack.push(Value::Number(left_val / right_val));
                }
                else {
                    panic!("This binary op was straight up illegal");
                }
                continue;
            }
        }
        println!("Uh oh, stinky!");
        return InterpretResult::RuntimeError;
    }
    return InterpretResult::Ok;
}






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

fn scan_text_and_make_tokens(source: &String, index: &mut usize, lines: &mut i64) -> Token {
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
            while *index + token_size < source.len() && source.chars().nth(*index + token_size).unwrap() != '\n' {
                token_size += 1;
            }
            println!("Comment starts at: {} and ends at: {}", *index, *index + token_size);
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
    globals: HashMap<String, Value>,
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

        println!("index: {}, data: \"{}\", tokentype: {:?}", old_index, token.data, token.token_type.clone());
        if token.token_type == TokenType::Error {
            succeeded = false;
            panic!("Token error while scanning!");
        }
        if token.token_type != TokenType::Comment {
            all_tokens.push(token);
        }
    }
    println!("Finished scanning of source code");
    return (succeeded, all_tokens);
}


fn emit_byte(chunk: &mut Chunk, byte: u8) -> usize {
    chunk.code.push(byte);
    return chunk.code.len() - 1;
}
fn emit_bytes(chunk: &mut Chunk, byte: u8, byte2: u8) -> usize {
    chunk.code.push(byte);
    chunk.code.push(byte2);
    return chunk.code.len() - 1;
}


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
        TokenType::BangEqual => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Equality}, 
        TokenType::Equal => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::EqualEqual => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Equality}, 
        TokenType::Greater => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Comparison}, 
        TokenType::GreaterEqual => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Comparison}, 
        TokenType::Less => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Comparison}, 
        TokenType::LessEqual => ParseRule {prefix: None, infix: Some(binary), precedence: Precedence::Comparison}, 
        TokenType::Identifier => ParseRule {prefix: Some(variable), infix: None, precedence: Precedence::None}, 
        TokenType::String => ParseRule {prefix: Some(string), infix: None, precedence: Precedence::None}, 
        TokenType::Number => ParseRule {prefix: Some(number), infix: None, precedence: Precedence::None},         
        TokenType::And => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Class => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Else => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::False => ParseRule {prefix: Some(literal), infix: None, precedence: Precedence::None}, 
        TokenType::For => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Fun => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::If => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Null => ParseRule {prefix: Some(literal), infix: None, precedence: Precedence::None}, 
        TokenType::Or => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Print => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Return => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Super => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::This => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::True => ParseRule {prefix: Some(literal), infix: None, precedence: Precedence::None}, 
        TokenType::Var => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::While => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 
        TokenType::Error => ParseRule {prefix: None, infix: None, precedence: Precedence::None}, 

        // THIS IS JUNK
        TokenType::Comment => ParseRule {prefix: Some(binary), infix: None, precedence: Precedence::None}, 
    };
    return rules[token_type];
}

fn parse_precedence(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize, precedence: Precedence) {
    println!("PRECEDENCE: {:?}, index: {}, precedence value of {}", precedence, *index, precedence as u8);
    *index += 1;
    let prefix_function = get_rule(all_tokens[*index-1].token_type).prefix;
    match prefix_function {
        Some(x) => x(chunk, all_tokens, index),
        None => println!("Prefix, expected expression!"),
    };

    while *index < all_tokens.len() {   
        if precedence as u8 <= get_rule(all_tokens[*index].token_type).precedence as u8 {
            let parse_rule = get_rule(all_tokens[*index].token_type);
            println!("infix time! token: {:?}, token_type: {:?}, index: {}, parse_rule here is {:?}", all_tokens[*index].data, all_tokens[*index].token_type, *index, parse_rule.precedence);
            *index += 1;
            match parse_rule.infix {
                Some(x) => x(chunk, all_tokens, index),
                None => (),
            };
        }
        else {
            println!("No proper infix at index: {}, precedence is {:?}", *index, get_rule(all_tokens[*index].token_type).precedence);
            return;
        }
    }
    println!("Index went out of range!");
}

fn create_string(chunk: &mut Chunk, string_token: &Token) {
    let mut stuff = vec![];
    let mut all_chars =  string_token.data.chars();
    all_chars.next();
    for _ in 0..string_token.data.len()-2 {
        stuff.push(all_chars.next().unwrap());
    }
    let value = Value::Obj(ObjData::String(stuff));
    add_constant(chunk, value, 0);
}

fn variable(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    let constant_index = identifier_constant(chunk, all_tokens, index);
    
    if advance_true_if_match(TokenType::Equal, all_tokens, index) {
        expression(chunk, all_tokens, index);
        emit_byte(chunk, OpCode::SetGlobal as u8);
    }
    else {
        emit_byte(chunk, OpCode::GetGlobal as u8);
    }
    emit_byte(chunk, constant_index);
}

fn string(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    match &all_tokens[*index-1].token_type {
        TokenType::String => create_string(chunk, &all_tokens[*index-1]),
        token_type => panic!("{:?} Not a literal, crashing", token_type),
    }
}

fn literal(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    match &all_tokens[*index-1].token_type {
        TokenType::True => emit_byte(chunk, OpCode::True as u8),
        TokenType::False => emit_byte(chunk, OpCode::False as u8),
        TokenType::Null => emit_byte(chunk, OpCode::Null as u8),
        token_type => panic!("{:?} Not a literal, crashing", token_type),
    };
}


fn number(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    let str_data = &all_tokens[*index-1].data;
    let value: Value = Value::Number(str_data.parse::<f64>().unwrap());
    println!("Running number at index {}, number is: {:?}", *index-1, value);
    add_constant(chunk, value, 0);
}

fn binary(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    println!("Running binary at index {}", *index);
    let last_token_type: TokenType = all_tokens[*index - 1].token_type;
    
    let rule: ParseRule = get_rule(last_token_type);
    parse_precedence(chunk, all_tokens, index, next_prec(rule.precedence));

    match last_token_type {
        TokenType::EqualEqual => emit_byte(chunk, OpCode::Equal as u8),
        TokenType::BangEqual => emit_bytes(chunk, OpCode::Equal as u8, OpCode::Not as u8),
        TokenType::Greater => emit_byte(chunk, OpCode::Greater as u8),
        TokenType::GreaterEqual => emit_bytes(chunk, OpCode::Less as u8, OpCode::Not as u8),
        TokenType::Less => emit_byte(chunk, OpCode::Less as u8),
        TokenType::LessEqual => emit_bytes(chunk, OpCode::Greater as u8, OpCode::Not as u8),
        TokenType::Plus => emit_byte(chunk, OpCode::Add as u8),
        TokenType::Minus => emit_byte(chunk, OpCode::Subtract as u8),
        TokenType::Star => emit_byte(chunk, OpCode::Multiply as u8),
        TokenType::Slash => emit_byte(chunk, OpCode::Divide as u8),
        _ => panic!("Not implemented lol"),
    };
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
    let lmao = format!("ERROR! Expected to consume tokentype \"{:?}\" at index: {}, but got {}", expected_token, *index, all_tokens[*index].data);
    println!("{}", lmao.red());
    println!("{}", error_message);
    return false;
}



fn expression(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    parse_precedence(chunk, all_tokens, index, Precedence::Assignment)
}

fn expression_statement(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    expression(chunk, all_tokens, index);
    consume(all_tokens, index, TokenType::Semicolon, "needed a semicolon here bud");
    emit_byte(chunk, OpCode::Pop as u8);
}

fn advance_true_if_match(token_type: TokenType, all_tokens: &Vec<Token>, index: &mut usize) -> bool {
    if *index >= all_tokens.len() || all_tokens[*index].token_type != token_type {
        return false;
    }
    *index += 1;
    return true;
}

fn print_statement(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    expression(chunk, all_tokens, index);
    emit_byte(chunk, OpCode::Print as u8);
    consume(all_tokens, index, TokenType::Semicolon, "needed a semicolon here bud");
}

fn if_statement(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    println!("If statement");
    expression(chunk, all_tokens, index);
    emit_byte(chunk, OpCode::JumpIfFalse as u8);
    let offset = emit_byte(chunk, 0xffu8);
    emit_byte(chunk, 0xffu8);
    
    statement(chunk, all_tokens, index);

    let jump = chunk.code.len() - (offset + 2);
    chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
    chunk.code[offset + 1] = (jump & 0xff) as u8;

    if advance_true_if_match(TokenType::Else, all_tokens, index) {
        statement(chunk, all_tokens, index);
    }
}

fn block(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    println!("block begin");
    while *index < all_tokens.len() && all_tokens[*index].token_type != TokenType::RightBrace {
        declaration(chunk, all_tokens, index);
    }
    println!("block end");
    consume(all_tokens, index, TokenType::RightBrace, "needed a semicolon here bud");
}

fn statement(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    if advance_true_if_match(TokenType::Print, all_tokens, index) {
        print_statement(chunk, all_tokens, index);
    }
    else if advance_true_if_match(TokenType::If, all_tokens, index) {
        if_statement(chunk, all_tokens, index);
    }
    else if advance_true_if_match(TokenType::LeftBrace, all_tokens, index) {
        block(chunk, all_tokens, index);
    }
    else {
        expression_statement(chunk, all_tokens, index);
    }
}

fn identifier_constant(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) -> u8{
    let last_token = &all_tokens[*index-1];
    let mut new_string = vec![];
    for i in last_token.data.chars() {
        new_string.push(i);
    }
    let var_name = Value::Obj(ObjData::String(new_string));
    return add_constant_dont_emit(chunk, var_name, 0);
}

fn parse_variable(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) -> u8 {
    consume(all_tokens, index, TokenType::Identifier, "Expected to see an identifier here for a variable name");
    return identifier_constant(chunk, all_tokens, index);
}

fn var_declaration(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    let global_constant_index = parse_variable(chunk, all_tokens, index);
    if advance_true_if_match(TokenType::Equal, all_tokens, index) {
        expression(chunk, all_tokens, index);
    }
    else {
        emit_byte(chunk, OpCode::Null as u8);
    }
    consume(all_tokens, index, TokenType::Semicolon, "needed a semicolon here bud");
    emit_byte(chunk, OpCode::DefineGlobal as u8);
    emit_byte(chunk, global_constant_index);
}

fn declaration(chunk: &mut Chunk, all_tokens: &Vec<Token>, index: &mut usize) {
    if advance_true_if_match(TokenType::Var, all_tokens, index) {
        println!("Lookin like a variable declaration aint it?");
        var_declaration(chunk, all_tokens, index);
    }
    else {
        println!("Just a statement");
        statement(chunk, all_tokens, index);
    }
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
    let mut index = 0;
    while index < all_tokens.len() {
        declaration(&mut chunk, &all_tokens, &mut index);
    }
    // emit_byte(&mut chunk, OpCode::Return as u8);
    return (true, chunk);
}


fn interpret(source: String) -> InterpretResult {
    let (_success, chunk) = compile(&source);
    return run(&mut VirtualMachine {
        chunk: chunk,
        ip: 0,
        stack: vec!(),
        globals: HashMap::new(),
    });
}

fn full_lines(mut input: impl std::io::BufRead) -> impl Iterator<Item = String> {
    std::iter::from_fn(move || {
        let mut final_string = String::new();
        let string_length = input.read_line(&mut final_string).unwrap();
        
        if string_length == 1 && final_string == "\n" {
            return None;
        }
        if string_length == 2  && final_string == "\r\n" {
            return None;
        }
        return Some(final_string);
    })
}

fn repl() {
    loop {
        print!("> ");
        let _result = std::io::stdout().flush();
        let mut source = String::new();
        for line in full_lines(std::io::stdin().lock()) {
            source += &line;
        }
        // let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        interpret(source);    
    }
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
