use std::env;
use std::io;

const USAGE: &str = "usage: computor_v1 | computor_v1 -c \"equation\"";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => loop_execution(),
        2 => execute_program(args[1].trim()),
        _ => display_usage(USAGE),
    }
}

fn loop_execution() {
    loop {
        println!("Enter an equation: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: reading stdin failed");

        let equation = input.trim();
        match equation {
            "quit" => break,
            _ => execute_program(equation),
        }
    }
}

fn display_usage(error: &str) {
    println!("{}", error);
}

fn execute_program(equation: &str) {
    println!("{}", equation);
}
