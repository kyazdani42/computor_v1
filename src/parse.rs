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
        _ => Err("wrong format."),
    }
}

// add parsing like 1 + 2 * x2 + 3 * x = 0 (current x2 is not supported)
fn parse_operations(operations: String) -> Result<Vec<Operation>, &'static str> {
    let mut operation_vec: Vec<Operation> = vec![];

    let lexed_operation: Vec<Token> = lex_operation(operations)?;
    let mut iterator = lexed_operation.clone();

    let mut last_parsed_index = 0;
    for (i, token) in iterator.iter_mut().enumerate() {
        let next_token = get_token_from_vec(&lexed_operation, i as i16 + 1);
        match token {
            Token::NUM(num) => {
                if next_token == Token::HAT {
                    return Err("Format error, there shouldn't be a ^ after a number");
                }
                match get_token_from_vec(&lexed_operation, i as i16 - 1) {
                    Token::NONE => {
                        let op = handle_operation_parsing(
                            &lexed_operation,
                            *num,
                            &mut last_parsed_index,
                            i as i16,
                        )?;
                        operation_vec.push(op);
                    }
                    Token::SIGN(sign) => {
                        if i as i16 - 2 == last_parsed_index {
                            if sign == '-' {
                                *num = -*num;
                            }
                            let op = handle_operation_parsing(
                                &lexed_operation,
                                *num,
                                &mut last_parsed_index,
                                i as i16,
                            )?;
                            operation_vec.push(op);
                        }
                    }
                    _ => {}
                }
            }
            Token::SIGN(_) => match next_token {
                Token::NUM(_) => {}
                _ => return Err("Format error on sign, next token must be a number"),
            },
            Token::MULT => {
                if next_token != Token::X {
                    return Err("Format error on *, next token must be *");
                }
            }
            Token::X => match next_token {
                Token::NONE | Token::HAT | Token::SIGN(_) => {}
                _ => return Err("Format error on x, next token must be *"),
            },
            Token::HAT => match next_token {
                Token::SIGN(_) | Token::NUM(_) => {}
                _ => return Err("Format error on ^, next token must be -/+ or a number"),
            },
            _ => return Err("lexer error, shouldn't get there"),
        }
    }
    Ok(operation_vec)
}

fn handle_operation_parsing(
    tokens: &Vec<Token>,
    current_number: f32,
    last_index: &mut i16,
    current_index: i16,
) -> Result<Operation, &'static str> {
    match get_token_from_vec(&tokens, current_index + 1) {
        Token::MULT => {}
        Token::SIGN(_) | Token::NONE => return Ok(Operation::new(current_number, 0.0)),
        _ => return Err("Format error."),
    };
    match get_token_from_vec(&tokens, current_index + 2) {
        Token::X => match get_token_from_vec(&tokens, current_index + 3) {
            Token::HAT => match get_token_from_vec(&tokens, current_index + 4) {
                Token::NUM(pow) => {
                    *last_index = current_index + 4;
                    Ok(Operation::new(current_number, pow))
                }
                Token::SIGN(sign) => match get_token_from_vec(&tokens, current_index + 5) {
                    Token::NUM(pow) => {
                        let mut pow = pow;
                        if sign == '-' {
                            pow = -pow;
                        }
                        *last_index = current_index + 5;
                        Ok(Operation::new(current_number, pow))
                    }
                    _ => Err("Format error."),
                },
                _ => Err("Format error."),
            },
            Token::SIGN(_) => {
                *last_index = current_index + 2;
                Ok(Operation::new(current_number, 1.0))
            }
            _ => Err("Format error."),
        },
        _ => Err("Format error."),
    }
}

fn get_token_from_vec(tokens: &Vec<Token>, i: i16) -> Token {
    if i < 0 || i > tokens.len() as i16 - 1 {
        Token::NONE
    } else {
        tokens[i as usize]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    NUM(f32),
    SIGN(char),
    X,
    HAT,
    MULT,
    NONE,
}

fn lex_operation(operation: String) -> Result<Vec<Token>, &'static str> {
    let mut lexer: Vec<Token> = vec![];
    let iterator = operation.bytes();
    let mut prev_str = String::new();
    for byte in iterator {
        let byte_is_not_num = !is_byte_numeral(byte);
        if byte_is_not_num && prev_str.len() != 0 {
            handle_number_lexing(&mut lexer, &mut prev_str)?;
        }
        match byte {
            b'x' | b'X' => lexer.push(Token::X),
            b'^' => lexer.push(Token::HAT),
            b'*' => lexer.push(Token::MULT),
            b'-' | b'+' => lexer.push(Token::SIGN(byte as char)),
            b'0'...b'9' | b'.' => prev_str.push(byte as char),
            b' ' | b'\n' => continue,
            _ => return Err("Found wrong character."),
        };
    }
    let must_parse_last_element = prev_str.len() != 0;
    if must_parse_last_element {
        handle_number_lexing(&mut lexer, &mut prev_str)?;
    };
    Ok(lexer)
}

fn handle_number_lexing(lexer: &mut Vec<Token>, value: &mut String) -> Result<(), &'static str> {
    let parsed_number: f32 = handle_float_parse_error(&value)?;
    lexer.push(Token::NUM(parsed_number));
    *value = String::new();
    Ok(())
}

fn is_byte_numeral(byte: u8) -> bool {
    match byte {
        b'0'...b'9' | b'.' => true,
        _ => false,
    }
}

fn handle_float_parse_error(value: &str) -> Result<f32, &'static str> {
    match value.parse() {
        Ok(v) => Ok(v),
        Err(_) => Err("cannot parse number"),
    }
}
