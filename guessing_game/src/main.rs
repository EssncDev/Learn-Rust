use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // build secret numbers 
    let secret_number = rand::thread_rng().gen_range(1..=30);

    println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");

        // assign guess as a mut new String
        let mut guess = String::new();

        // get terminal input [Format: int]
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert guess to a u32 and parse value
        let guess: u32 = guess.trim().parse(){
            Ok(num) => num.
            Err(_) => continue,
        }

        println!("You guessed: {guess}");

        // compare the guess and evaluate condition
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