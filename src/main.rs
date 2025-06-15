use std::io;

fn main() {
    let mut name = String::new();
    println!("Please inout your name.");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name from stdin");

    // Remove newline
    let name = name.trim();

    println!("Hello {name}!");
}
