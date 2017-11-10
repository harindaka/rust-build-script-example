fn main() {
    
    static STRING_ONE: &'static str = include_str!(concat!(env!("OUT_DIR"), "/hello_world.txt"));
    static STRING_TWO: &'static str = include_str!(concat!(env!("OUT_DIR"), "/gen_file.txt"));

    println!("{}", STRING_ONE);
    println!("{}", STRING_TWO);
}
