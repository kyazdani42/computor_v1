use std::io;

use parse;
use compute;
use equation;

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
    let simplified_operation = compute::simplify(equation);
    println!("Reduced form: {} = 0", equation::get_str_from_vec(&simplified_operation));
    // get equation level and compute
}
