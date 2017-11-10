fn main() {
    
    static LONG_STRING: &'static str = include_str!("file_path.txt");
    println!("{}", LONG_STRING);
}
