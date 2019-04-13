use equation::{Equation, Operation};

pub fn parse(s: String) -> Result<Equation, &'static str> {
    let cleaned_str = retain_spaces(&s);
    let operations = split_equal(&cleaned_str)?;

    let left_op = parse_operations(operations[0].to_owned())?;
    let right_op = parse_operations(operations[1].to_owned())?;
    Ok(Equation::new(left_op, right_op))
}

fn split_equal(s: &str) -> Result<Vec<&str>, &'static str> {
    let operations: Vec<&str> = s.split('=').filter(| v | v.len() != 0).collect();
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

fn find_token(lexed_operation: Vec<Lexer>, i: i16) -> Lexer {
    match lexed_operation
        .iter()
        .clone()
        .enumerate()
        .find(|( j, _ )| *j as i16 == i)
    {
        Some(v) => v.1.clone(),
        None => Lexer::NONE
    }
}

fn parse_operations(operations: String) -> Result<Vec<Operation>, &'static str> {
    let lexed_operation: Vec<Lexer> = lex_operation(operations)?;
    let index_max = lexed_operation.len() - 1;
    let mut operation_vec: Vec<Operation> = vec![];
    let mut val: Option<f32> = None;
    let mut pow: Option<i16> = None;
    for (i, token) in lexed_operation.iter().enumerate() {
        let next_token = find_token(lexed_operation.clone(), i as i16 + 1);
        let previous_token = find_token(lexed_operation.clone(), i as i16 - 1);
        match token {
            Lexer::SIGN(_) => {
                match previous_token {
                    Lexer::SIGN(_) => return Err("Format error."),
                    Lexer::HAT => continue,
                    _ => {}
                };
                if i == index_max { return Err("Format error.") }
                if i != 0 {
                    if pow == None {
                        if previous_token == Lexer::X {
                            pow = Some(1);
                        } else if val != None {
                            match previous_token {
                                Lexer::NUM(v) => pow = Some(v as i16),
                                _ => return Err("Format error.")
                            }
                        } else {
                            pow = Some(0);
                        }
                    }
                    if val == None {
                        match previous_token {
                            Lexer::NUM(v) => val = Some(v),
                            _ => return Err("Format error.")
                        }
                    }
                    operation_vec.push(Operation::new(val.unwrap(), pow.unwrap()));
                    val = None;
                    pow = None;
                }
            },
            Lexer::NUM(num) => {
                let mut value = *num;
                match previous_token {
                    Lexer::NONE => val = Some(value),
                    Lexer::X | Lexer::HAT => pow = Some(value as i16),
                    Lexer::SIGN(sign) => if sign == '-' { value = -value; },
                    Lexer::MULT => return Err("Format error."),
                    _ => return Err("lexer error, shouldn't get there"),
                }
                if i == index_max {
                    if pow == None && val == None {
                        pow = Some(0);
                        val = Some(value);
                    } else if pow == None {
                        pow = Some(value as i16);
                    }
                    operation_vec.push(Operation::new(val.unwrap(), pow.unwrap()));
                }
            },
            Lexer::X => {
                match previous_token {
                    Lexer::MULT => return Err("format error, previous token must be *. (x)"),
                    _ => {}
                };
                if i == index_max {
                    operation_vec.push(Operation::new(val.unwrap(), 1));
                }
            },
            Lexer::HAT => {
                match previous_token {
                    Lexer::X => {},
                    _ => return Err("format error, previous token must be x. (mult)")
                };
                match next_token {
                    Lexer::SIGN(_) | Lexer::NUM(_) => {},
                    _ => return Err("format error, next token must be sign or number. (mult)")
                };
            },
            Lexer::MULT => {
                match previous_token {
                    Lexer::NUM(num) => val = Some(num),
                    _ => return Err("format error, previous token must be number. (mult)")
                }
                match next_token {
                    Lexer::X => {},
                    _ => return Err("format error, next token must be x. (mult)")
                }
            },
            _ => return Err("lexer error, shouldn't get there")
        }
    }
    println!("{:?}", operation_vec);
    Ok(operation_vec)
}

#[derive(Debug, PartialEq, Clone)]
enum Lexer {
    NUM(f32),
    SIGN(char),
    X,
    HAT,
    MULT,
    NONE,
}

fn lex_operation(operation: String) -> Result<Vec<Lexer>, &'static str> {
    let mut lexer: Vec<Lexer> = vec![];
    let iterator = operation.bytes();
    let mut prev_str = String::new();
    for byte in iterator {
        let byte_is_not_num = !is_byte_numeral(byte);
        if byte_is_not_num && prev_str.len() != 0 { handle_number_lexing(&mut lexer, &mut prev_str)?; }
        match byte {
            b'x' => lexer.push(Lexer::X),
            b'^' => lexer.push(Lexer::HAT),
            b'*' => lexer.push(Lexer::MULT),
            b'-' | b'+' => lexer.push(Lexer::SIGN(byte as char)),
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