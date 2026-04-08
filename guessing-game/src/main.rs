use guessing_game::guessing_game::Guess;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,     // Return input if it is a u32 number
            Err(_) => continue, // Ignore invalid inputs and continue the game
        };

        // Though the Guess type already has internal validation in the new() method
        // We use this check here because guessing_game is a user-facing program and
        // crashing on an invalid USER input will be bad UX for the user.
        //
        // Though the ideal way to handle this situation would've been to resturn a
        // Resullt<Guess, Error> from Guess::new() instead of just a Guess with
        // panic in case of invalid input.
        //
        // The book's example is mainly to teach this distinction only, so this
        // program will be left as it is.
        if !(1..=100).contains(&guess) {
            println!("A value between 1 and 100 is expected!");
            continue;
        }

        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break; // Quit when the guess is correct.
            }
        }
    }
}
