use equation::{Equation, Operation};
use std::str::Bytes;

const ERR_FORMAT: &str = "Wrong Format !";

pub fn parse(s: String) -> Result<Equation, &'static str> {
    let operations = split_equal(&s)?;

    let left_op = get_operation_vec(operations[0])?;
    let right_op;
    if operations[1].trim() != "0" {
        right_op = get_operation_vec(operations[1])?;
    } else {
        right_op = vec![Operation::new(0, 0)];
    };

    Ok(Equation::new(left_op, right_op))
}

fn split_equal(s: &str) -> Result<Vec<&str>, &'static str> {
    let operations: Vec<&str> = s.split('=').collect();
    match operations.len() {
        2 => Ok(operations),
        _ => Err(ERR_FORMAT),
    }
}

fn get_operation_vec(operation: &str) -> Result<Vec<Operation>, &'static str> {
    match parse_operations(retain_spaces(operation)) {
        Some(vec) => Ok(vec),
        None => Err(ERR_FORMAT),
    }
}

fn retain_spaces(operation: &str) -> String {
    let mut cleaned_operation = String::from(operation).to_lowercase();
    cleaned_operation.retain(|c| c != ' ');
    cleaned_operation
}

fn parse_operations(operations: String) -> Option<Vec<Operation>> {
    let mut result: Vec<Operation> = Vec::new();
    let mut index: usize = 0;
    loop {
        let sliceable_operations = String::from(&operations[index..]);

        let iterator = sliceable_operations.bytes();
        let operation_result = get_operation_from_iterator(iterator)?;
        index += operation_result.index;

        let operation = get_operation_from_str(&operation_result.value, operation_result.operator)?;
        result.push(operation);

        if index >= operations.len() {
            break;
        };
    }

    Some(result)
}

#[derive(Debug, PartialEq)]
struct OperationIterationResult {
    index: usize,
    operator: Sign,
    value: String,
}

impl OperationIterationResult {
    fn new() -> OperationIterationResult {
        OperationIterationResult {
            index: 0,
            value: String::new(),
            operator: Sign::Pos,
        }
    }
}

fn get_operation_from_iterator(iterator: Bytes) -> Option<OperationIterationResult> {
    let mut return_value = OperationIterationResult::new();
    for byte in iterator {
        match byte {
            b'0'...b'9' | b'x' | b'^' | b'*' => {
                return_value.value.push(byte as char);
            }
            b'-' | b'+' => {
                if return_value.index > 0 {
                    break;
                }
                if byte == b'-' {
                    return_value.operator = Sign::Neg;
                }
            }
            _ => return None,
        };
        return_value.index += 1;
    }
    Some(return_value)
}

#[derive(Debug, PartialEq)]
enum Sign {
    Pos,
    Neg,
}

fn get_operation_from_str(operation_as_str: &str, sign: Sign) -> Option<Operation> {
    let splitted_by_mult: Vec<&str> = operation_as_str.split('*').collect();
    if splitted_by_mult.len() != 2 {
        return None;
    };
    let wrapped_num = splitted_by_mult.first()?.parse();
    let value: i64;
    if wrapped_num.is_err() {
        return None;
    } else {
        value = wrapped_num.unwrap();
    };
    let splitted_by_pow: Vec<&str> = splitted_by_mult.last()?.split('^').collect();
    if splitted_by_pow.len() != 2 || splitted_by_pow.first()? != &"x" {
        return None;
    };
    let wrapped_pow = splitted_by_pow.last()?.parse::<i16>();

    let pow;
    if wrapped_pow.is_err() {
        return None;
    } else {
        pow = wrapped_pow.unwrap();
    };
    if pow > 2 {
        return None;
    }
    let neg = match sign {
        Sign::Neg => -1,
        Sign::Pos => 1,
    };
    Some(Operation::new(value * neg, pow))
}

// -------------- TESTS ----------------- //

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fail_split_equal() {
        let result = split_equal("test without equal");
        assert_eq!(result.err(), Some(ERR_FORMAT))
    }

    #[test]
    fn test_split_equal() {
        let left_operator: &str = split_equal("test = test2").unwrap().first().unwrap();
        let right_operator: &str = split_equal("test = test2").unwrap().last().unwrap();
        let left_ok = left_operator == "test ";
        let right_ok = right_operator == " test2";
        assert!(left_ok && right_ok)
    }

    #[test]
    fn test_retain_spaces() {
        let cleaned = retain_spaces("test  te s t t es t ");
        let expected = String::from("testtesttest");
        assert_eq!(cleaned, expected)
    }

    #[test]
    fn test_operation_from_iterator() {
        let test_value = "14*x^1+13*x^2".bytes();
        let expected = OperationIterationResult {
            index: 6,
            operator: Sign::Pos,
            value: "14*x^1".to_owned(),
        };
        let result = get_operation_from_iterator(test_value).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_operation_from_iterator_minus() {
        let test_value = "-14*x^1+13*x^2".bytes();
        let expected = OperationIterationResult {
            index: 7,
            operator: Sign::Neg,
            value: "14*x^1".to_owned(),
        };
        let result = get_operation_from_iterator(test_value).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn fail_operation_from_iterator() {
        let test_value = "14*ax^1+1*2".bytes();
        let result = get_operation_from_iterator(test_value);
        assert_eq!(result, None)
    }

    #[test]
    fn test_get_operation() {
        let test_value = "14*x^1";
        let result = get_operation_from_str(test_value, Sign::Pos).unwrap();
        let expected = Operation::new(false, 14, 1);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_operation_neg() {
        let test_value = "14*x^2";
        let result = get_operation_from_str(test_value, Sign::Neg).unwrap();
        let expected = Operation::new(true, 14, 2);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_operation_fail() {
        let test_value = "14*x^";
        let result = get_operation_from_str(test_value, Sign::Pos);
        assert!(result.is_none())
    }
}
