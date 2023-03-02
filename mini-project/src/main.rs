use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n = input.trim().parse().expect("Invalid number");

    let mut fib = vec![0, 1];
    while fib.last().unwrap() < &n {
        let next = fib[fib.len() - 1] + fib[fib.len() - 2];
        fib.push(next);
    }

    println!("Fibonacci sequence up to {}: {:?}", n, fib);
}
