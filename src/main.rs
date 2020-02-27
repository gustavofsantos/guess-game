extern crate rand;

//bring io library to the file scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // println! is a macro
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    loop {
        println!("Please input your guess (1-10).");
    
        // variables are immutable (unless declared with mut)
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid. Your input should be a number");
                    continue;
                },
            };
        
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}