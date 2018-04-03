use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate clap;

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

    let paths = fs::read_dir(dir_name).unwrap();

    let mut contents = String::new();

    println!("With text:\n{}", contents);

    for path in paths {
        let new_content = get_file(&path.unwrap().path());
        contents.push_str(&new_content.unwrap())

    }

    Ok(contents)
}

// Todo: Receive input dir and output file as parameters from the command line
fn main() {

    let matches = App::new("Concatenate")
        .version("0.0.1")
        .author("Natu Lauchande <nlauchande@gmail.com>")
        .about("Concatenate tool for the command line in rust")
        .arg(Arg::with_name("DIR")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("directory"))
        .get_matches();

    let dir = matches.value_of("DIR").unwrap();

    let dir = "./resources";

    let output_file = "output.concatenate";

    let contents = concat_dir(dir).unwrap();

    let mut file = File::create(output_file);

    file.unwrap().write_all(contents.as_bytes());

}
