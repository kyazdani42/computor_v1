use std::io;

use parse;

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

pub fn run(equation: String) {
    let parsed_value = match parse::parse(equation) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    println!("at the end: {}", parsed_value);
    // simplify
    // if 2 compute delta
    // else calculate
    // calculat
}
