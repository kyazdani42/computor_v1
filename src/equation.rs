use std::fmt;

pub struct Equation {
    pub l_op: Vec<Operation>,
    pub r_op: Vec<Operation>,
}

impl Equation {
    pub fn new(l_op: Vec<Operation>, r_op: Vec<Operation>) -> Equation {
        Equation { l_op, r_op }
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let l_op = get_str_from_vec(&self.l_op);
        let r_op = get_str_from_vec(&self.r_op);
        write!(f, "{}= {}", l_op, r_op)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    pub value: f32,
    pub pow: f32,
}

impl Operation {
    pub fn new(value: f32, pow: f32) -> Operation {
        Operation { value, pow }
    }

    pub fn abs(&self) -> f32 {
        if self.value < 0.0 {
            -self.value
        } else {
            self.value
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_value = self.abs();
        if self.pow == 0.0 {
            return write!(f, "{}", display_value);
        };
        if self.pow == 1.0 {
            return write!(f, "{} * X", display_value);
        };
        write!(f, "{} * X^{}", display_value, self.pow)
    }
}

pub fn get_str_from_vec(vec: &Vec<Operation>) -> String {
    let mut s = String::new();
    for (i, op) in vec.iter().enumerate() {
        let sign = if op.value < 0.0 {
            "-"
        } else if i > 0 {
            "+"
        } else {
            ""
        };
        let formatted_operation = format!("{} {}", sign, op);
        s.push_str(&formatted_operation);
        s.push(' ');
    }
    if s.len() == 0 {
        " 0 ".to_owned()
    } else {
        s
    }
}
