use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Guess Game!!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please enter a number between 1 and 100:");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please enter a valid number");
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win! The secret number was: {secret_number}")
            }
        }
    }
}
