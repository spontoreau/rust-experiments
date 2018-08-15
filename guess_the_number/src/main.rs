extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guess the number! \nRules: 5 turns to retrieve a number between 1 and 100");

    let secret = rand::thread_rng().gen_range(1, 101);
    let mut turn = 1;

    while turn <= 5 {
        println!("Turn {} - Please enter your guess:", turn);
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Read line error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        println!("Your guess {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Great job's, you win!");
                break;
            }
        }

        turn = turn + 1;
    }

    println!("Game over!");
}
