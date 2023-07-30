use std::{
    env::{self},
    fs, process,
    sync::Arc,
    thread,
};

use crate::files::process_file;

mod args;
mod files;

fn main() {
    let mut threads = vec![];
    let args = env::args();
    let args = args::parse_args(args);
    let args = Arc::new(args);
    let is_dir = fs::metadata(&args.path)
        .unwrap_or_else(|err| {
            eprintln!("Could not open file: {}", err);
            process::exit(1);
        })
        .is_dir();

    let is_file = fs::metadata(&args.path)
        .unwrap_or_else(|err| {
            eprintln!("Could not open file: {}", err);
            process::exit(1);
        })
        .is_file();

    if is_dir {
        let files = files::get_file_paths(&args.path, args.recursive).unwrap_or_else(|err| {
            eprintln!("Could not get file paths: {}", err);
            process::exit(1);
        });

        for file in files {
            let args = Arc::clone(&args);
            let search_thread = thread::spawn(move || {
                let content = fs::read_to_string(&file);
                match content {
                    Ok(content) => {
                        process_file(&content, &args);
                    }
                    Err(_) => (),
                }
            });
            threads.push(search_thread);
        }
    }

    if is_file {
        let content = fs::read_to_string(&args.path).unwrap_or_else(|err| {
            eprintln!("Could not open file: {:?}", err);
            process::exit(1);
        });
        process_file(&content, &args);
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
