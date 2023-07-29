use std::env::{self, Args};
mod paths;

fn main() {
    let args = env::args();
    parse_args(args);
    let files = paths::get_file_paths("./", true).unwrap();

    for file in files {
        println!("{:?}", file);
    }
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

Example: hermes ./Users
Example: hermes username  ./Users.txt


-h, --help            Print help.
-i, --ignore-case     Ignore case.
"
    )
}
