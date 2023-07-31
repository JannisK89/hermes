use std::process::exit;

#[derive(Debug, PartialEq)]
pub struct ArgOptions {
    pub sub_str: String,
    pub path: String,
    pub recursive: bool,
    pub ignore_case: bool,
}

pub fn parse_args(mut args: Box<dyn Iterator<Item = String>>) -> ArgOptions {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_returns_argoptions() {
        let args_one = Box::new(
            vec![
                String::from("Skip me"),
                String::from("substring"),
                String::from("path"),
                String::from("-i"),
                String::from("-r"),
            ]
            .into_iter(),
        );
        let args_two = Box::new(
            vec![
                String::from("Skip me"),
                String::from("substring"),
                String::from("path"),
            ]
            .into_iter(),
        );

        let args_one = parse_args(args_one);
        let args_two = parse_args(args_two);

        let expected_one = ArgOptions {
            sub_str: String::from("substring"),
            path: String::from("path"),
            ignore_case: true,
            recursive: true,
        };
        let expected_two = ArgOptions {
            sub_str: String::from("substring"),
            path: String::from("path"),
            ignore_case: false,
            recursive: false,
        };

        assert_eq!(args_one, expected_one);
        assert_eq!(args_two, expected_two);
    }
}
