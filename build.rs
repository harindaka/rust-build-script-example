extern crate build_script_file_gen;
use build_script_file_gen::gen_file_str;

use std::{env};
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello_world.txt");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    let hello_world = "Hello World!";
    write!(f, "{}", hello_world).unwrap();

    //The same functionality above can be achieved using 
    //the build_script_file_gen crate's gen_file_str method
    let file_content = "This content was added and included using the build_script_file_gen crate's gen_file function and include_file_str! macro respectively";
    gen_file_str("gen_file.txt", &file_content);
}