use std::io::{self, Write};

fn main() {
    let mut string = String::new();
    print!("Please enter a string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut string).unwrap();
    let string = string.trim(); 
    let uppercase = string.to_uppercase();
    let lowercase = string.to_lowercase();
    println!("Uppercase: {}", uppercase);
    println!("Lowercase: {}", lowercase);
}
