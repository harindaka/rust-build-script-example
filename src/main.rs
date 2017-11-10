fn main() {
    
    static LONG_STRING: &'static str = include_str!("long_string.txt");
    println!("{}", LONG_STRING);
}
