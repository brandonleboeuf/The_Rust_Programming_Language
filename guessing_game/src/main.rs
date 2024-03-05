use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! (Hint, its between 1 and 100...");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // // Check if the user wants to crash the program by typing any letter
        if guess.trim().len() >= 1 && guess.trim().chars().next().unwrap().is_alphabetic() {
            panic!("That's no number, lil Chudd... You need to leave...!");
        }

        // Continue with the number guessing logic
        let guess: i32 = match guess.trim().parse::<i32>() {
            Ok(num) => num + 2,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, what the hell Chudd!"),
            Ordering::Greater => println!("Too big, my Chudd!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
