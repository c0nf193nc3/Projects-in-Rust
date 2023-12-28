use std::io;

// This program converts a decimal number to binary.
fn main() {
    let mut decimal = String::new();

    println!("Enter a decimal number:");
    io::stdin().read_line(&mut decimal)
        .expect("Failed to read line");

    let decimal: u32 = decimal.trim().parse()
        .expect("Please type a number!");

    let binary = format!("{:b}", decimal);
    println!("The binary equivalent of decimal {} is {}.", decimal, binary);
}
