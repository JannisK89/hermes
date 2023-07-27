use std::{
    env::{self, Args},
    fs::{self},
};

fn main() {
    let args = env::args();
    parse_args(args);
    let mut files: Vec<String> = vec![];
    read_directory("./", &mut files);
    println!("FILES:\n{:?}", files);
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

fn read_directory(dir: &str, files: &mut Vec<String>) {
    let directory = fs::read_dir(dir).unwrap();
    for file in directory {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        let is_file = file_type.is_file();

        if is_file {
            files.push(String::from(dir.to_owned() + file.path().to_str().unwrap()));
        } else {
            read_directory(file.path().to_str().unwrap(), files);
        }
    }
}
