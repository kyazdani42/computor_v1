use equation::{Equation, Operation};

pub fn parse(s: String) -> Result<Equation, &'static str> {
    let operations = split_equal(&s)?;

    let left_op = parse_operations(operations[0].to_owned())?;
    let right_op = parse_operations(operations[1].to_owned())?;
    Ok(Equation::new(left_op, right_op))
}

fn split_equal(s: &str) -> Result<Vec<&str>, &'static str> {
    let operations: Vec<&str> = s.split('=').filter(|v| v.len() != 0).collect();
    match operations.len() {
        2 => Ok(operations),
        _ => Err("Format error."),
    }
}

fn parse_operations(operations: String) -> Result<Vec<Operation>, &'static str> {
    let mut operation_vec: Vec<Operation> = vec![];

    let lexed_operation: Vec<Token> = lex_operation(operations)?;
    let iterator = lexed_operation.clone();

    let mut pow: Option<f32> = None;
    let mut value: Option<f32> = None;
    for (i, token) in iterator.iter().enumerate() {
        let next_token = get_token_from_vec(&lexed_operation, i as i16 + 1);
        match token {
            Token::NUM(num) => match next_token {
                Token::MULT => {
                    if value.is_some() {
                        pow = Some(*num);
                    } else {
                        value = Some(*num);
                    }
                },
                Token::NONE | Token::NUM(_) => {
                    if value.is_some() {
                        pow = Some(*num);
                    } else {
                        pow = Some(0.0);
                        value = Some(*num);
                    }
                },
                _ => return Err("Format error."),
            },
            Token::HAT => match next_token {
                Token::NUM(_) => continue,
                _ => return Err("Format error.")
            },
            Token::MULT => if next_token != Token::X {
                return Err("Format error.")
            },
            Token::X => match next_token {
                Token::NONE | Token::NUM(_) => {
                    if !value.is_some() {
                        value = Some(1.0);
                    }
                    pow = Some(1.0);
                }
                Token::HAT => {},
                _ => return Err("Format error.")
            },
            _ => return Err("Format error."),
        }
        if pow.is_some() && value.is_some() {
            operation_vec.push(Operation::new(value.unwrap(), pow.unwrap()));
            pow = None;
            value = None;
        }
    }
    Ok(operation_vec)
}

fn get_token_from_vec(tokens: &Vec<Token>, i: i16) -> Token {
    if i < 0 || i > tokens.len() as i16 - 1 {
        Token::NONE
    } else {
        tokens[i as usize]
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Token {
    NUM(f32),
    X,
    HAT,
    MULT,
    NONE,
}

fn lex_operation(mut operation: String) -> Result<Vec<Token>, &'static str> {
    operation.retain(| v | v != ' ' && v != '\n');
    let mut lexer: Vec<Token> = vec![];
    let iterator = operation.bytes();
    let mut prev_str = String::new();
    for byte in iterator {
        let should_be_tokenized = !is_byte_num(byte) || is_byte_sign(byte);
        if (prev_str == "-" || prev_str == "+") && byte == b'X' || byte == b'x' {
            if prev_str == "-" {
                lexer.push(Token::NUM(-1.0));
                lexer.push(Token::MULT);
            } else {
                lexer.push(Token::NUM(1.0));
                lexer.push(Token::MULT);
            }
            prev_str = String::new();
        } else if should_be_tokenized && prev_str.len() != 0 {
            prev_str = handle_number_lexing(&mut lexer, &prev_str)?;
        }
        match byte {
            b'x' | b'X' => lexer.push(Token::X),
            b'^' => lexer.push(Token::HAT),
            b'*' => lexer.push(Token::MULT),
            b'0'...b'9' | b'.' | b'-' | b'+' => prev_str.push(byte as char),
            _ => return Err("Format error."),
        };
    }
    let must_parse_last_element = prev_str.len() != 0;
    if must_parse_last_element {
        handle_number_lexing(&mut lexer, &mut prev_str)?;
    };
    Ok(lexer)
}

fn handle_number_lexing(lexer: &mut Vec<Token>, value: &String) -> Result<String, &'static str> {
    let parsed_number: f32 = handle_float_parse_error(&value)?;
    lexer.push(Token::NUM(parsed_number));
    Ok(String::new())
}

fn is_byte_sign(byte: u8) -> bool {
    match byte {
        b'+' | b'-' => true,
        _ => false,
    }
}

fn is_byte_num(byte: u8) -> bool {
    match byte {
        b'0'...b'9' | b'.' | b'+' | b'-' => true,
        _ => false,
    }
}

fn handle_float_parse_error(value: &str) -> Result<f32, &'static str> {
    match value.parse() {
        Ok(v) => Ok(v),
        Err(_) => Err("Format error."),
    }
}
