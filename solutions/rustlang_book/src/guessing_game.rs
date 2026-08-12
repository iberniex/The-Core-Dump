use std::cmp::Ordering;
use std::io;

pub use self::guessing_game_function as guessing_game;

use rand;
#[derive(Debug)]
pub struct Guess {
    value: i64,
}

impl Guess {
    pub fn new(value: i64) -> Guess {
        if value < 1 || value > 100 {
            panic!("The value should be in the range: 0-100 \n");
        }

        Guess { value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

pub fn guessing_game_function() {
    println!("Guess the number");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = rand::random_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
