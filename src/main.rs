use std::io;

fn main() {
    println!("Hello, world. Going to wait...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
