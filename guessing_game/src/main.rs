extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess a number!");

        // the secret number is given a random number thread_rng becomes a generator
        // gen_range is the generated range
        let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess");

        // allows the guess to become mutable due to it being an input from the user
        let mut guess = String::new();

        // this reads the input from the first line 
        // .expect is set up as an error handler 
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // converts the guess to a string so that it can be compared to the secret number
        // Also handles the error that if it is anything other than a number it will keep asking
        // rather than crashing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };   
        println!("You guessed: {}", guess);
    
    // match uses an arm pattern in order to compare two values
    // an arm consists of a pattern and the code to be ran
    // it takes the value given to match and looks through the arm patterns in turn
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }  
    }       
}

