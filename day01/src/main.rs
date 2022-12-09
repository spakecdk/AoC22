use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", "../input.txt");

    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
