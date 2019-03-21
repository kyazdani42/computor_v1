use std::io;

use parse;
use compute;

pub fn run_loop() {
    loop {
        println!("Enter an equation: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: reading stdin failed");

        let equation = input.trim();
        match equation {
            "quit" | "q" => break,
            "" => continue,
            _ => run(equation.to_owned()),
        }
    }
}

pub fn run(input: String) {
    let equation = match parse::parse(input) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    println!("equation: {}", equation);
    let simplified_equation = match compute::simplify(equation) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    println!("simplified: {}", simplified_equation);
    // if 2 compute delta
    // else calculate
    // calculat
}
