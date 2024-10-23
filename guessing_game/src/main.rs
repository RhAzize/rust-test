use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    // test tuple
    tuple();

    // test array
    array();
    // test guess
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

// Tuple
fn data() -> (u32, u32, String) {
    let x: u32 = 5;
    let y: u32 = 10;
    let z: String = "Hello".to_string();
    (x, y, z)
}

fn array() {
    //Array----------------------------------------
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] = {},a[1] = {}", a[0], a[1]);
    //----------------------------------------------
}

fn tuple() {
    // On déstructure le tuple
    let (x, y, z) = data();
    println!("data value x:{x} y={y} z={z}");

    // On ignore des valeurs
    let (x, _, z) = data();
    println!("data value without y x:{x} z={z}");
}
