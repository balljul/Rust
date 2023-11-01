use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");
    
    let mut guess = String::new();
    let number_to_guess = rand::thread_rng().gen_range(1..=5); 

    println!("Please input your guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your input");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Your number was: {guess}");

    match guess.cmp(&number_to_guess){
        Ordering::Less => println!("Guess Higher"),
        Ordering::Greater => println!("Guess lower"),
        Ordering::Equal => println!("You guessed right"),
    } 

}
