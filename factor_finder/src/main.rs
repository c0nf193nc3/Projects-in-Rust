use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter a number: ");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let num: u128 = input.trim().parse().unwrap();

    println!("Factors of {} are:", num);

    for i in 1..=num {
        if num % i == 0 {
            println!("{}", i);
        }
    }
}
