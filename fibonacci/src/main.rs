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
    if n == 0 {
        return 0;
    } 
    else if n == 1 {
        return 1;
    }
    else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
