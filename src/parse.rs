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
        _ => Err("Format error, missing some parts of the equation."),
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
                    if pow.is_some() {
                        return Err("Format error, * cannot be placed here.");
                    }
                    if value.is_some() {
                        pow = Some(*num);
                    } else {
                        value = Some(*num);
                    }
                }
                Token::NONE | Token::NUM(_) => {
                    if value.is_some() {
                        pow = Some(*num);
                    } else {
                        pow = Some(0.0);
                        value = Some(*num);
                    }
                }
                _ => return Err("Format error, number must be followed either by nothing, a * or a sign."),
            },
            Token::HAT => match next_token {
                Token::NUM(_) => continue,
                _ => return Err("Format error, ^ must be followed by number."),
            },
            Token::MULT => {
                if next_token != Token::X {
                    return Err("Format error, * must be followed by X.");
                }
            }
            Token::X => match next_token {
                Token::NONE | Token::NUM(_) => {
                    if !value.is_some() {
                        value = Some(1.0);
                    }
                    pow = Some(1.0);
                }
                Token::HAT => {}
                _ => return Err("Format error, X can be followed either by ^, a number or nothing."),
            },
            _ => return Err("Program error, shouldn't get there."),
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

// ------------------- LEXING ---------------------------- //

#[derive(PartialEq, Clone, Copy)]
enum Token {
    NUM(f32),
    X,
    HAT,
    MULT,
    NONE,
}

fn lex_operation(operation: String) -> Result<Vec<Token>, &'static str> {
    let mut lexer: Vec<Token> = vec![];
    let mut prev_str = String::new();

    let byte_vector: Vec<u8> = operation.bytes().collect();
    let mut i = 0;
    for byte in &byte_vector {
        if *byte == b'\n' || *byte == b' ' {
            if prev_str.len() != 0 && i != byte_vector.len() - 1 && is_byte_num(&byte_vector[i + 1]) && prev_str != "-" && prev_str != "+" {
                return Err("Format error, numbers can't be separated by a space.");
            }
            i += 1;
            continue;
        }

        if byte_is_x(byte) {
            if i < byte_vector.len() - 1 {
                let new_iterator = &byte_vector[i + 1..];
                let next_char = new_iterator.iter().find(| v | **v != b' ' && **v != b'\n');
                if next_char.is_some() && *next_char.unwrap() != b'+' && *next_char.unwrap() != b'-' && *next_char.unwrap() != b'^' {
                    return Err("Format error, X must be followed by a sign, a ^, or nothing.")
                }
            }
            if is_str_sign(&prev_str) || i == 0 {
                prev_str = add_1_mult_before_x(&mut lexer, &prev_str);
            }
        }

        let should_be_tokenized = !is_byte_num(byte);
        if should_be_tokenized && prev_str.len() != 0 {
            prev_str = handle_number_lexing(&mut lexer, &prev_str)?;
        }

        match byte {
            b'x' | b'X' => lexer.push(Token::X),
            b'^' => lexer.push(Token::HAT),
            b'*' => lexer.push(Token::MULT),
            b'0'...b'9' | b'.' | b'-' | b'+' => prev_str.push(*byte as char),
            _ => return Err("Format error, found wrong character."),
        };
        i += 1;
    }
    let must_parse_last_element = prev_str.len() != 0;
    if must_parse_last_element {
        handle_number_lexing(&mut lexer, &mut prev_str)?;
    };
    Ok(lexer)
}

fn byte_is_x(byte: &u8) -> bool {
    *byte == b'X' || *byte == b'x'
}

fn is_str_sign(num_as_str: &str) -> bool {
    num_as_str == "+" || num_as_str == "-"
}

fn is_byte_num(byte: &u8) -> bool {
    match byte {
        b'0'...b'9' | b'.' => true,
        _ => false,
    }
}

fn handle_float_parse_error(value: &str) -> Result<f32, &'static str> {
    match value.parse() {
        Ok(v) => Ok(v),
        Err(_) => Err("Format error, cannot parse number."),
    }
}

fn handle_number_lexing(lexer: &mut Vec<Token>, value: &String) -> Result<String, &'static str> {
    let parsed_number: f32 = handle_float_parse_error(&value)?;
    lexer.push(Token::NUM(parsed_number));
    Ok(String::new())
}

fn add_1_mult_before_x(lexer: &mut Vec<Token>, prev_str: &str) -> String {
    if prev_str == "-" {
        lexer.push(Token::NUM(-1.0));
        lexer.push(Token::MULT);
    } else {
        lexer.push(Token::NUM(1.0));
        lexer.push(Token::MULT);
    };
    String::new()
}
