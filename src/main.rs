use std::io;

fn init() {
    println!("Guess the number");
    println!("Please input your guess")
}

fn start() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    print!("You guessed {guess}");    
}

fn main() {
    init();
    start();
}