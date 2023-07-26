use std::env::{self, Args};

fn main() {
    let args = env::args();
    parse_args(args);
}

fn parse_args(mut args: Args) {
    args.next();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => print_help(),
            _ => (),
        }
    }
}

fn print_help() {
    println!(
        "
Usage:
hermes SUBSTRING  ./PATH
hermes SUBSTRING  ./PATH_TO_FILE
Example: hermes username -f ./Users


-h, --help            Print help.
-u, --ignore-case     Ignore case.
"
    )
}
