use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Creates a random int
    let secret_Number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess: ");

        let mut guess = String::new(); // mut = mutable variable

        // let apples = 5 -> inmutable variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You have guessed: {guess}");

        match guess.cmp(&secret_Number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
