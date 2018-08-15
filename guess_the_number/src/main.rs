use std::io;

fn main() {
    println!("Welcome to guess the number! \nRules: 5 turns to retrieve a number between 1 and 100");

    println!("Please enter your guess:");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Read line error");

    println!("Your guess {}", guess);
}
