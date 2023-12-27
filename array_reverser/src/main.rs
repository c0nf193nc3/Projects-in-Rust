use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the number of elements in the array: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    let mut array = vec![0; n];
    for i in 0..n {
        print!("Enter element {}: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        array[i] = input.trim().parse().unwrap();
        input.clear();
    }

    array.reverse();
    println!("{:?}", array);
}
