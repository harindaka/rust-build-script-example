fn main() {
    
    static LONG_STRING: &'static str = include_str!(concat!(env!("OUT_DIR"), "/hello_world.txt"));

    println!("{}", LONG_STRING);
}
