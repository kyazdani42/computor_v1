mod app;
mod equation;
mod parse;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(v) => app::run(v),
        None => app::run_loop()
    }
}
