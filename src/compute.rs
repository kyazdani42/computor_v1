use equation::{Equation, Operation};

pub fn simplify(operation: Equation) -> Vec<Operation> {
    let mut reversed_r_op = operation
        .r_op
        .into_iter()
        .map(|mut v| {
            v.value = -v.value;
            v
        })
        .collect();
    let mut operation = operation.l_op.clone();
    operation.append(&mut reversed_r_op);
    let filtered_op = operation.into_iter().filter(|v| v.value != 0.0).collect();
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
        .filter(|x| x.value != 0.0)
        .collect()
}

pub fn resolve(operation: &Vec<Operation>) -> String {
    let higher_polynomial = operation
        .iter()
        .fold(0, |a, b| if a > b.pow { a } else { b.pow });
    println!("Polynomial degree: {}", higher_polynomial);
    if higher_polynomial > 2 {
        return "The polynomial degree is stricly greater than 2, I can't solve.".to_owned();
    }
    let pow2 = get_operation_value_from_pow(&operation, 2);
    let pow1 = get_operation_value_from_pow(&operation, 1);
    let constant = get_operation_value_from_pow(&operation, 0);
    if pow2 == 0.0 {
        linear_operation(pow1, constant)
    } else {
        quadratic_operation(pow2, pow1, constant)
    }
}

pub fn linear_operation(pow1: f32, constant: f32) -> String {
    let constant_null = constant == 0.0;
    let pow1_null = pow1 == 0.0;

    if pow1_null && constant_null {
        "Every value is a solution to the equation".to_owned()
    } else if constant_null {
        format!("The solution is {}", -pow1)
    } else {
        format!(
            "The solution is
{}",
            -(constant / pow1)
        )
    }
}
pub fn sqrt_babylon(num: f32) -> f32 {
    if num < 0.0 {
        return 0.0;
    };
    let mut x = num / 4.0;
    let mut e = (num - (x * x)) / 2.0 * x;
    while x != e {
        e = x;
        x = (x + (num / x)) / 2.0;
    }
    x
}

pub fn quadratic_operation(a: f32, b: f32, c: f32) -> String {
    let discriminant = (b * b) - (4.0 * a * c);
    let divider = 2.0 * a;
    let bneg = -b;
    if discriminant > 0.0 {
        let root_square = sqrt_babylon(discriminant);
        format!(
            "Discriminant is strictly positive, the two solutions are:\n{}\n{}",
            (bneg + root_square) / divider,
            (bneg - root_square) / divider
        )
    } else if discriminant < 0.0 {
        let root_square = sqrt_babylon(-discriminant);
        let first_root = format!("({} - i{}) / {}", bneg, root_square, divider);
        let second_root = format!("({} + i{}) / {}", bneg, root_square, divider);
        format!(
            "No Real solution for the equation, the two complexe solutions are:\n{}\n{}",
            first_root, second_root
        )
    } else {
        format!("{} = x", -(b / 2.0 * a))
    }
}

fn get_operation_value_from_pow(operation: &Vec<Operation>, pow: i16) -> f32 {
    match operation.iter().find(|v| v.pow == pow) {
        Some(v) => v.value,
        None => 0.0,
    }
}
