use std::{fs, io};

use crate::args::ArgOptions;

// Gets path for all files in a folder and subfolder if recursive is true
pub fn get_file_paths(dir: &str, recursive: bool) -> Result<Vec<String>, io::Error> {
    let mut files: Vec<String> = vec![];
    if let Ok(directory) = fs::read_dir(dir) {
        for file in directory {
            let file = file?;
            let file_type = file.file_type()?;
            let is_file = file_type.is_file();

            if is_file {
                files.push(String::from(file.path().to_str().unwrap_or_else(|| {
                    eprintln!("Could not convert file path to string: {:?}", file.path());
                    ""
                })));
            } else {
                if recursive {
                    let path = file.path();
                    let path_str = path.to_str().unwrap_or_else(|| {
                        eprintln!("Could not convert file path to string");
                        ""
                    });
                    let rec_files = get_file_paths(path_str, true)?;
                    for file in rec_files {
                        files.push(file);
                    }
                }
            }
        }
    }
    Ok(files)
}

fn find_substr(line: &str, substr: &str, ignore_case: bool) -> Option<String> {
    if !ignore_case && line.contains(substr) {
        let marked = String::from("\x1b[91;4m".to_owned() + substr + "\x1b[0m");
        let replaced = line.replace(substr, &marked);
        return Some(replaced);
    } else if ignore_case && line.to_lowercase().contains(&substr.to_lowercase()) {
        let marked = String::from("\x1b[91;4m".to_owned() + substr + "\x1b[0m");
        let replaced = line.to_lowercase().replace(&substr.to_lowercase(), &marked);
        return Some(replaced);
    }
    None
}

pub fn process_file(content: &str, args: &ArgOptions) {
    let split_content: Vec<&str> = content.split("\n").collect();
    for line in split_content {
        if let Some(substr) = find_substr(line, &args.sub_str, args.ignore_case) {
            println!("{}", substr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_substring_returns_marked_lines() {
        let line_one = "You have power over your mind - not outside events. Realize this, and you will find strength.";
        let line_two =
            "dwell on the beauty of life. watch the stars, and see yourself running with them.";

        let expected_one = "You have \x1b[91;4mpower\x1b[0m over your mind - not outside events. Realize this, and you will find strength.";
        let expected_two =
            "dwell on the \x1b[91;4mBEAUTY\x1b[0m of life. watch the stars, and see yourself running with them.";

        let result_one = find_substr(line_one, "power", false).unwrap();
        let result_two = find_substr(line_two, "BEAUTY", true).unwrap();

        assert_eq!(result_one, expected_one);
        assert_eq!(result_two, expected_two);
    }
}
