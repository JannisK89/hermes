use std::{
    env::{self},
    fs,
};
mod args;
mod paths;

fn main() {
    let args = env::args();
    let args = args::parse_args(args);
    let files = paths::get_file_paths(&args.path, args.recursive).unwrap();

    for file in files {
        println!("{:?}", file);
        let content = fs::read_to_string(&file);
        match content {
            Ok(content) => {
                println!("{file} contains:");
                println!("{content}\n\n");
            }
            Err(_) => (),
        }
    }
}
