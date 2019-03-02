use equation::{Equation, Operation};

const ERR_FORMAT: &str = "Wrong Format !";

pub fn parse(s: String) -> Result<Equation, &'static str> {
    let operations = split_equal(&s)?;

    let left_op = get_operation_vec(operations[0])?;
    let right_op = get_operation_vec(operations[1])?;

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

// string in form a*x^pow(+|-)b*x^pow(+|-)c*x^pow
fn parse_operations(operations: String) -> Option<Vec<Operation>> {
    println!("{}", operations);
    let mut result: Vec<Operation> = Vec::new();

    result.push(Operation::new());

    Some(result)
}

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
}
