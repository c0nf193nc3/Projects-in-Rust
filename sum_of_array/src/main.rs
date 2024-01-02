use std::io::{self};

fn main() {
    let mut numbers = Vec::new();

    println!("Enter numbers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for num_str in input.split_whitespace() {
        let num: i32 = num_str.parse().unwrap();
        numbers.push(num);
    }

    let sum: i32 = numbers.iter().sum();
    println!("The sum of the array is: {}", sum);
}
