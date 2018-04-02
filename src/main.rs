use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let folder = "/";

    let ouput = "output.concatenate";

    let mut buffer = String::new();

    let paths = fs::read_dir("./resources").unwrap();

    let mut contents = String::new();

    println!("With text:\n{}", contents);

    for path in paths {

         let mut new_contents = String::new();
         let mut f = File::open(&path.unwrap().path()).expect("file not found");

         f.read_to_string(&mut new_contents)
             .expect("something went wrong reading the file");

        contents.push_str(&new_contents)

    }
    println!("{}",contents);
}
