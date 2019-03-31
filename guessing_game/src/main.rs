use std::io; // Import standard io library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101); // Generate random number between 1 and 101.
    println!("The random number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Declare mutable variable and initialize as new String.

        io::stdin().read_line(&mut guess) // Returns instance of std::io::Stdin (handle to stdin of terminal)
        // read_line reads a line from stdin.
            .expect("failed to read line"); // catch io error.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(thing) => {
                println!("{}", thing);
                continue;
            }, // use _ to signify unused or non-existant value.
        };
        
        println!("You guessed: {}", guess); // String formatting to print result.
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        } // Compare random number to guess.

    }
}
