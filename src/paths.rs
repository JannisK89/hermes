use std::{fs, io};

// Gets path for all files in a folder and subfolder if recursive is true
pub fn get_file_paths(dir: &str, recursive: bool) -> Result<Vec<String>, io::Error> {
    let mut files: Vec<String> = vec![];
    let directory = fs::read_dir(dir)?;
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
    Ok(files)
}
