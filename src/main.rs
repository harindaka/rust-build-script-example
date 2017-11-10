#[macro_use]
extern crate build_script_file_gen;

fn main() {
    
    static STRING_ONE: &'static str = include_str!(concat!(env!("OUT_DIR"), "/hello_world.txt"));    
    println!("{}", STRING_ONE);
    
    //The same functionality above can be achieved using 
    //the build_script_file_gen crate's include_file_str! macro
    println!(include_file_str!("hello.txt"));

    //or

    //Includes the file content in the surrounding code unhygienically.
    include_file!("hello.rs");
}
