extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
        // Generate a random number between 1 and 100
    let secret = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret){
            Ordering::Equal => {
                println!("You won");
                break;
            },
            Ordering::Greater => println!("Your number is greater"),
            Ordering::Less => println!("Your number is smaller"),
        }
    }
}