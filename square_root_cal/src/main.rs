use std::io::{self, Write};

fn main() {
    let mut num = String::new();

    print!("Please enter a number: ");
    
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).unwrap();
    
    let num: f64 = num.trim().parse().unwrap(); 
    
    let sqrt = num.sqrt();

    println!("The square root of {} is {}.", num, sqrt);
}
