use std::{fmt,clone};

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

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub negative: bool,
    pub value: i64,
    pub pow: i16,
}

impl Operation {
    pub fn new(negative: bool, value: i64, pow: i16) -> Operation {
        Operation {
            negative,
            value,
            pow,
        }
    }
    pub fn clone_invert(&self) -> Operation {
        Operation {
            negative: !self.negative,
            value: self.value,
            pow: self.pow
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.pow == 0 {
            return write!(f, "{}", self.value);
        };
        let sign = if self.negative { "-" } else { "+" };
        if self.pow == 1 {
            return write!(f, "{} {} * X", sign, self.value);
        };
        write!(f, "{} {} * X^{}", sign, self.value, self.pow)
    }
}

impl clone::Clone for Operation {
    fn clone(&self) -> Operation {
        Operation {
            negative: self.negative,
            value: self.value,
            pow: self.pow
        }
    }
}

pub fn get_str_from_vec(vec: &Vec<Operation>) -> String {
    let mut s = String::new();
    for op in vec {
        let formatted_operation = format!("{}", op);
        s.push_str(&formatted_operation);
        s.push(' ');
    }
    s
}
