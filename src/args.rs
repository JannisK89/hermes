use std::{env::Args, process::exit};

pub struct ArgOptions {
    pub sub_str: String,
    pub path: String,
    pub recursive: bool,
    pub ignore_case: bool,
}

pub fn parse_args(mut args: Args) -> ArgOptions {
    let mut recursive = false;
    let mut ignore_case = false;
    args.next();
    let sub_str = args.next().unwrap_or_else(|| {
        eprintln!("Invalid amount of arguments");
        print_help();
        exit(1);
    });
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Invalid amount of arguments");
        print_help();
        exit(1);
    });
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => print_help(),
            "-r" | "--recursive" => recursive = true,
            "-i" | "--ignore-case" => ignore_case = true,
            _ => (),
        }
    }

    ArgOptions {
        sub_str,
        path,
        recursive,
        ignore_case,
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
-r, --recursive       Recursive search in subdirectories
-i, --ignore-case     Ignore case.
"
    )
}
