use rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    //Gen un num de 0 a 100
    let secret: u32 = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Guess the number!\nPlease input your guess.");
        io::stdout().flush().unwrap();

        // On créer une var mutable ac une capacité de 10
        let mut guess = String::with_capacity(10);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // On convertit la string en u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Puis on compare le guess avecc le secret
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small! 🤬"),
            Ordering::Greater => println!("Too big! 😂"),
            Ordering::Equal => {
                println!("You WIN!!!🦀");
                break;
            }
        }
        println!("You guessed: {}", guess);
    }
}
