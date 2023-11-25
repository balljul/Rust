use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Number must between 1 and 100");
        }
        Guess {value}
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess a number!");
    
    let number_to_guess = rand::thread_rng().gen_range(1..=100); 

    loop {
        let mut guess = String::new();

        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&number_to_guess){
            Ordering::Less => println!("Guess Higher"),
            Ordering::Greater => println!("Guess lower"),
            Ordering::Equal => {
                println!("You guessed right");
                break;
            }
        } 
    }

}
