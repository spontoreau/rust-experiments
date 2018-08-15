extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to guess the number! \nRules: 5 turns to retrieve a number between 1 and 100");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Please enter your guess:");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Read line error");

    let guess: u32 = guess.trim().parse().expect("Not a number");

    println!("Your guess {}", guess);
}
