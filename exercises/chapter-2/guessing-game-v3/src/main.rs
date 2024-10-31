use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // shadowing the previous guess variable
        // rather than creating two separate variables
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // if the user enters a non-number
                // the program will continue to the next iteration
                println!("Please enter a valid number.");
                continue;
            }
        };
        println!("You guessed: {}", guess);
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
