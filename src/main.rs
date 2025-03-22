use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate clap;
extern crate tempfile;

use clap::{Arg, App};

fn  get_file(path: &Path) -> std::io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn concat_dir(dir_name: &str) -> std::io::Result<String> {
    let paths = fs::read_dir(dir_name)?;
    
    // Collect and sort file paths for consistent output
    let mut file_paths = Vec::new();
    for path in paths {
        file_paths.push(path?.path());
    }
    file_paths.sort();
    
    let mut contents = String::new();
    
    for file_path in file_paths {
        if file_path.is_file() {
            if let Ok(new_content) = get_file(&file_path) {
                contents.push_str(&new_content);
                contents.push('\n');
            } else {
                eprintln!("Warning: Could not read file: {:?}", file_path);
            }
        }
    }

    Ok(contents)
}

fn main() {

    let matches = App::new("Concatenate")
        .version("0.0.1")
        .author("Natu Lauchande <nlauchande@gmail.com>")
        .about("Concatenate tool for the command line in rust")
        .arg(Arg::with_name("DIR")
            .short("d")
            .long("dir")
            .required(true)
            .index(1)
            .takes_value(true)
            .help("directory with files to be concatenated"))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .required(true)
            .index(2)
            .takes_value(true)
            .help("name of the output file with all the contents merged"))
        .get_matches();

    let dir = matches.value_of("DIR").unwrap();

    let output_file = matches.value_of("OUTPUT").unwrap();

    match concat_dir(dir) {
        Ok(contents) => {
            match File::create(output_file) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(contents.as_bytes()) {
                        eprintln!("Error writing to output file: {}", e);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("Error creating output file: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("Error reading from directory: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;
    extern crate tempfile;
    use tempfile::tempdir;

    #[test]
    fn test_concat_dir() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        
        // Create test files
        let file1_path = dir.path().join("file1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "Content of file 1").unwrap();
        
        let file2_path = dir.path().join("file2.txt");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "Content of file 2").unwrap();
        
        // Test the function
        let result = concat_dir(dir.path().to_str().unwrap()).unwrap();
        
        // Check that result contains both files' contents
        assert!(result.contains("Content of file 1"));
        assert!(result.contains("Content of file 2"));
    }
}
