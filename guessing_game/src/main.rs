extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 100);
    let mut guesses = 0;
    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        // let guess: i32 = guess.trim().parse().expect("Please enter a number");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(num) => {
                println!("{}", num);
                continue;
            }
        };

        println!("You guessed: {}", guess);
        guesses += 1;

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                    println!("You won in {} guesses!", guesses);
                    break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
