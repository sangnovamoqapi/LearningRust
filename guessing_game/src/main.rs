use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Game starts!");
    println!("Enter your guess");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");
    println!("You guessed {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let mut compare = guess.cmp(&secret_number);

    while compare != Ordering::Equal {
        if compare == Ordering::Less {
            println!("Too small!");
        }
        if compare == Ordering::Greater {
            println!("Too big!");
        }

        println!("Guess is wrong: Guess another number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        compare = guess.cmp(&secret_number);
    }
    println!("You guessed it! The secret number is indeed {secret_number}")
}