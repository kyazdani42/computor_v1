use equation::{Equation,Operation};

pub fn simplify(operation: Equation) -> Vec<Operation> {
  let operation_right = operation.r_op[0].clone_invert();
  let mut operation_left = operation.l_op.clone();
  if operation_right.value != 0 {
    operation_left.push(operation_right);
  }
  get_simplified_operation(operation_left)
}

fn get_simplified_operation(operation: Vec<Operation>) -> Vec<Operation> {
  let mut simplified_operation: Vec<Operation> = vec![];
  for v in operation.iter() {
    let mut should_push = true;
    for s in simplified_operation.iter_mut() {
      if v.pow == s.pow {
        should_push = false;
        let v_value = if v.negative == true { v.value * -1 } else { v.value };
        let s_value = if s.negative == true { s.value * -1 } else { s.value };
        s.value = v_value + s_value;
      };
    };
    if should_push == true {
      simplified_operation.push(v.clone());
    }
  }
  simplified_operation.into_iter().filter(| x | { x.value != 0 }).collect()
}
