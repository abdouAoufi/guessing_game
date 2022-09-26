use rand::Rng;
use std::{cmp::Ordering, io};

fn init() {
    println!("Guess the number");
    println!("Please input your guess")
}

fn start() {
    let mut guess = String::new();
    let random_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {random_number}");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    print!("You guessed {guess}");
    compare(random_number, guess)
}

fn compare(random_number: i32, guess: String) {
    let guess: i32 = guess.trim().parse().expect("Should be a number");
    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}

fn main() {
    init();
    start();
}
