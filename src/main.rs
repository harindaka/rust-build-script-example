fn main() {
    
    static LONG_STRING: &'static str = include_str!(concat!(env!("OUT_DIR"), "/file_path.txt"));

    println!("{}", LONG_STRING);
}
