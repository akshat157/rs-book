use std::io;

fn main() {
    println!("Enter n to get the nth fibonacci number:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u8 = n.trim().parse().expect("Invalid input. Expected a positive number.");

    let nth_fib = fib_bottom_up(n);
    println!("{n}th fibonacci number using bottom-up approach is: {nth_fib}");

    let nth_fib = fib_recursive(n);
    println!("{n}th fibonacci number using recursive approach is: {nth_fib}");

}

fn fib_recursive(n: u8) -> u64 {
    if n <= 1 { return u64::from(n); }
    fib_recursive(n-1) + fib_recursive(n-2);
}

fn fib_bottom_up(n: u8) -> u64 {
    if n <= 1 { return u64::from(n); }

    let mut i: u8 = 2;
    let mut x = 0u64;
    let mut y = 1u64;

    while i <= n {
        let z = y; 
        y = x + y;
        x = z;
        i += 1; 
    }

    y
}

