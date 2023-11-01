use std::io;

fn main() {
    println!("Guess a number!");
    
    let mut guess = String::new();
    let number_to_guess = 5;

    println!("Please input your guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your input");

    println!("Your number was: {guess}The right number would be: {number_to_guess}");
}
