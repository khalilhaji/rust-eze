use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
		println!("Guess the number!");

		let secret_number = rand::thread_rng().gen_range(1, 101);


		loop {
				println!("Please input your guess.");

				let mut guess = String::new();

				match io::stdin().read_line(&mut guess) {
						Err(v) => panic!("Failed to read line! {}", v),
						Ok(_) => (),
				}

				let guess: u32 = match guess.trim().parse() {
						Ok(i) => i,
						_ => {
								println!("Please type a valid number!");
								continue;
						},
				};
				
				

				println!("You guessed: {}", guess);

				match guess.cmp(&secret_number) {
						Ordering::Less => println!("Too small"),
						Ordering::Equal => {println!("You win!"); break;},
						Ordering::Greater => println!("Too big"),
				}
		}
}
