// Import the io module and the Write trait from the standard library
use std::io::{self, Write};

// The main function where your program starts execution
fn main() {
    // Declare a mutable variable `input` and initialize it with an empty string
    let mut input = String::new();

    // Print the string "Enter a number: " to the console
    print!("Enter a number: ");

    // Flush the output to the console
    io::stdout().flush().unwrap();

    // Read a line from the standard input (the user's input) and append it to the `input` string
    io::stdin().read_line(&mut input).unwrap();

    // Remove any leading or trailing whitespace from the `input` string
    // Attempt to convert the string to a number
    // Declare a variable `num` and store the parsed number in it
    let num: u128 = input.trim().parse().unwrap();

    // Print "Factors of {num} are:", where `{num}` is replaced with the value of `num`
    println!("Factors of {} are:", num);

    // Start a loop that iterates over the numbers from 1 to `num` (inclusive)
    for i in 1..=num {
        // Check if `num` is divisible by `i` (i.e., if `num` modulo `i` is 0)
        if num % i == 0 {
            // If `num` is divisible by `i`, print `i`
            println!("{}", i);
        }
    }
}
