use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the principal: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let principal: f64 = input.trim().parse().unwrap();
    input.clear();

    print!("Enter the rate: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let rate: f64 = input.trim().parse().unwrap();
    input.clear();

    print!("Enter the time: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let time: f64 = input.trim().parse().unwrap();

    let interest = principal * rate * time;
    println!("The interest on the loan is {}.", interest);
}