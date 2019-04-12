use equation::{Equation, Operation};
pub fn simplify(operation: Equation) -> Vec<Operation> {
    let mut reversed_r_op = operation
        .r_op
        .into_iter()
        .map(|mut v| {
            v.value = v.value * -1;
            v
        })
        .collect();
    let mut operation = operation.l_op.clone();
    operation.append(&mut reversed_r_op);
    let filtered_op = operation.into_iter().filter(|v| v.value != 0).collect();
    get_simplified_operation(filtered_op)
}

fn get_simplified_operation(operation: Vec<Operation>) -> Vec<Operation> {
    let mut simplified_operation: Vec<Operation> = vec![];
    for orig in operation.iter() {
        let mut should_push = true;
        for new in simplified_operation.iter_mut() {
            if orig.pow == new.pow {
                should_push = false;
                new.value = orig.value + new.value;
            };
        }
        if should_push == true {
            simplified_operation.push(orig.clone());
        }
    }
    simplified_operation
        .into_iter()
        .filter(|x| x.value != 0)
        .collect()
}

pub fn resolve(operation: &Vec<Operation>) -> String {
    let c = get_operation_value_from_pow(&operation, 0);
    let b = get_operation_value_from_pow(&operation, 1);
    let a = get_operation_value_from_pow(&operation, 2);
    if a == 0 {
        linear_operation(b, c)
    } else {
        quadratic_operation(a, b, c)
    }
}

pub fn linear_operation(a: i64, b: i64) -> String {
    if a == 0 {
        if b == 0 {
            "Every value is a solution to the equation".to_owned()
        } else {
            "No solution for the equation".to_owned()
        }
    } else {
        format!("{} = x", -(b as f32 / a as f32))
    }
}

pub fn quadratic_operation(a: i64, b: i64, c: i64) -> String {
    let discriminant = (b * b) - (4 * a * c);
    if discriminant > 0 {
        let root_square = (discriminant as f32).sqrt();
        let divider = 2.0 * a as f32;
        let bneg = -b as f32;
        format!(
            "{} = x and {} = x",
            (bneg + root_square) / divider,
            (bneg - root_square) / divider
        )
    } else if discriminant < 0 {
        //TODO complexe roots info
        "No real solution for the equation, discriminant is inferior to 0".to_owned()
    } else {
        format!("{} = x", -(b as f32 / (2 * a) as f32))
    }
}

fn get_operation_value_from_pow(operation: &Vec<Operation>, pow: i16) -> i64 {
    match operation.iter().find(|v| v.pow == pow) {
        Some(v) => v.value,
        None => 0,
    }
}
