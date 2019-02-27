use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let no_args: bool = args.len() == 1;
    let args_not_correct: bool = args.len() != 3 || args[1].trim() != "-c";
    if no_args {
        loop {
            println!("Enter the equation: ");
            let mut equation = String::new();
            io::stdin().read_line(&mut equation).expect("error: reading stdin failed");
            execute_program(equation);
        }
    } else if args_not_correct {
        let usage = "usage: computor_v1 | computor_v1 -c \"equation\"";
        print_error(&usage);
    } else {
        let equation: String = args[2].clone();
        execute_program(equation);
    }
}

fn print_error(error: &str) {
    println!("{}", error);
}

fn execute_program(equation: String) {
    println!("{}", equation);
}

// 1) lexing / parsing equation
// 2) simplify
// 3) compute
// 4) display
