use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Result;

// A simple implementation of `% cat path`
fn  get_file(path: &Path) -> std::io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("Hello, world!");

    let folder = "/";

    let ouput = "output.concatenate";

    let mut buffer = String::new();

    let paths = fs::read_dir("./resources").unwrap();

    let mut contents = String::new();

    println!("With text:\n{}", contents);

    for path in paths {
        let new_content = get_file(&path.unwrap().path());
        contents.push_str(&new_content.unwrap())

    }


    println!("{}",contents);
}
