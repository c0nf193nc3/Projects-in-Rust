use std::io;

fn main() {
    let mut num = String::new();

    println!("Please enter the number, which you want to check for even or odd.");

    io::stdin().read_line(&mut num)
        .expect("Failed to read line");

    let num: i32 = num.trim().parse()
        .expect("Please type a number!");

    let is_even = num % 2 == 0;

    if is_even {
        println!("The number {} is even", num);
    } else {
        println!("The number {} is odd", num);
    }
}
