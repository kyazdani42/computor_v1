use std::fmt;

pub struct Equation {
    pub l_op: Vec<Operation>,
    pub r_op: Vec<Operation>
}

#[derive(Debug)]
pub struct Operation {
    pub negative: bool,
    pub value: i64,
    pub pow: i16,
}

impl Equation {
  pub fn new(l_op: Vec<Operation>, r_op: Vec<Operation>) -> Equation {
    Equation { l_op, r_op }
  }
}

impl Operation {
  pub fn new() -> Operation {
    Operation { negative: false, value: 0, pow: 0 }
  }
}

impl fmt::Display for Equation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let l_op = get_str_from_vec(&self.l_op);
    let r_op = get_str_from_vec(&self.r_op);
    write!(f, "{} = {}", l_op, r_op)
  }
}

impl fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let sign = if self.negative {
      "-"
    } else {
      "+"
    };
    write!(f, "{}{} * X^{}", sign, self.value, self.pow)
  }
}

fn get_str_from_vec(vec: &Vec<Operation>) -> String {
  let mut s = String::new();
  for op in vec {
      let formatted_operation = format!("{}", op);
      s.push_str(&formatted_operation);
      s.push(' ');
  }
  s
}
