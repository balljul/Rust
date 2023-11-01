use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    
    let mut guess = String::new();

    let number_to_guess = rand::thread_rng().gen_range(1..=20); 
    // (1..20) Means exclusive 20
    // (1..=20) Means inclusive 20

    println!("Please input your guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your input");

    println!("Your number was: {guess}The right number would be: {number_to_guess}");
}
