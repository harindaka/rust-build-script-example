use std::{iter, env};
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("long_string.txt");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    let long_string = dest_path.display();
    write!(f, "{}", long_string).unwrap();
}