extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //Generates a random number on the main thread
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop /* Loops forever */ { 
        println!("Please input your guess.");

        let mut guess = String::new();

        //Read in the users guess
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        //If the user inputs a number assign it,
        //Else ask the user to input a number 
        let guess: u32 = match guess.trim().parse() {
            Ok(num)  => num,
            Err(_)   => continue,
        };

        println!("You guessed {}", guess);

        //Compare the guessed number to the random number
        //If they are the same end the game
        match guess.cmp(&secret_number) {
        Ordering::Less       => println!("Too Small!"),
        Ordering::Greater    => println!("Too Big!"),
        Ordering::Equal      => {
                                    println!("You Win!");
                                    break;
                                }
        }
    }
}
