use std::io::{self, Write};

fn main() {

    println!("Guess the number!\n Please input your guess.");
    io::stdout().flush().unwrap();

    let mut guess = String::with_capacity(10);

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess.trim());
}
