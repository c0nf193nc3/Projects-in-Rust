use std::io::{self, Write};

fn main() {
    let mut string = String::new();
    let mut times = String::new();

    print!("Please enter a string: ");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut string).unwrap();

    print!("Please enter a number: ");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut times).unwrap();

    let string = string.trim();

    match times.trim().parse::<usize>() {
        Ok(t) => {
            let repeated = string.repeat(t);
            println!("{}", repeated);
        },
        Err(_) => {
            println!("Invalid number entered. Please try again with a valid number.");
        }
    }
}
