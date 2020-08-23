use std::io;

fn main() {
    println!("Going to wait...");
    io::stdin().read_line(&mut String::new()).unwrap();
}