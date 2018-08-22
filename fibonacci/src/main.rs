use std::io;

fn main() {
    println!("Enter a value");
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Can't read line");

    let n: i32 = n.trim().parse().expect("Not a number");

    let result = fibonacci(n);
    println!("Result: {}", result);
}

fn fibonacci(n: i32) -> i32 {
    return match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    };
}
