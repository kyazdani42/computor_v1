use equation::{Equation, Operation};

pub fn parse(s: String) -> Result<Equation, &'static str> {
    let operations = split_equal(&s)?;

    let left_op = parse_operations(retain_spaces(operations[0]))?;
    let right_op = parse_operations(retain_spaces(operations[1]))?;
    Ok(Equation::new(left_op, right_op))
}

fn split_equal(s: &str) -> Result<Vec<&str>, &'static str> {
    let operations: Vec<&str> = s.split('=').collect();
    match operations.len() {
        2 => Ok(operations),
        _ => Err("wrong format."),
    }
}

fn retain_spaces(operation: &str) -> String {
    let mut cleaned_operation = String::from(operation).to_lowercase();
    cleaned_operation.retain(|c| c != ' ' && c != '\n');
    cleaned_operation
}

fn parse_operations(operations: String) -> Result<Vec<Operation>, &'static str> {
    let lexed_operation: Vec<Lexer> = lex_operation(operations)?;
    let mut operation_vec: Vec<Operation> = vec![Operation::new(0.0, 0)];
    for token in lexed_operation.iter() {
        println!("{:?}", token)
    }
    Ok(operation_vec)
}

#[derive(Debug)]
enum Lexer {
    NUM(f32),
    OP(char),
    UNK,
    HAT,
    MULT,
}

fn lex_operation(operation: String) -> Result<Vec<Lexer>, &'static str> {
    let mut lexer: Vec<Lexer> = vec![];
    let iterator = operation.bytes();
    let mut prev_str = String::new();
    for byte in iterator {
        let byte_is_not_num = !is_byte_numeral(byte);
        if byte_is_not_num && prev_str.len() != 0 { handle_number_lexing(&mut lexer, &mut prev_str)?; }
        match byte {
            b'x' => lexer.push(Lexer::UNK),
            b'^' => lexer.push(Lexer::HAT),
            b'*' => lexer.push(Lexer::MULT),
            b'-' | b'+' => lexer.push(Lexer::OP(byte as char)),
            b'0'...b'9' | b'.' => prev_str.push(byte as char),
            _ => return Err("Found wrong character."),
        };
    }
    let must_parse_last_element = prev_str.len() != 0;
    if must_parse_last_element { handle_number_lexing(&mut lexer, &mut prev_str)?; };
    Ok(lexer)
}

fn handle_number_lexing(lexer: &mut Vec<Lexer>, value: &mut String) -> Result<(), &'static str> {
    let parsed_number: f32 = handle_float_parse_error(&value)?;
    lexer.push(Lexer::NUM(parsed_number));
    *value = String::new();
    Ok(())
}

fn is_byte_numeral(byte: u8) -> bool {
    match byte {
        b'0'...b'9' | b'.' => true,
        _ => false
    }
}

fn handle_float_parse_error(value: &str) -> Result<f32, &'static str> {
    match value.parse() {
        Ok(v) => Ok(v),
        Err(_) => Err("cannot parse number")
    }
}