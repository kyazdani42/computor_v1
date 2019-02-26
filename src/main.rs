use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let no_args = args.len() == 1;
    let args_not_correct = args.len() != 3 || args[1].trim() != "-c";
    if no_args {
        loop {
            // might wan't to do the same execution than in else, just add an stdin read first
        }
    } else if args_not_correct {
        println!("usage: computor_v1 | computor_v1 -c \"equation\"");
    } else {
        let equation = &args[2];
        println!("{}", equation);
    }
}

// 1) lexing / parsing equation
// 2) simplify
// 3) compute
// 4) display
