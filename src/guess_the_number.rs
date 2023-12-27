use std::io;
use rand::Rng;
use std::cmp::Ordering;

#[allow(non_snake_case)]
#[allow(dead_code)]

fn guessNumber() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess!");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
fn main() {
    guessNumber();
    loop {
        println!("Play again? (y/n)");
        let mut encore = String::new();
        io::stdin()
            .read_line(&mut encore)
            .expect("Failed to read guess!");
        if encore.to_lowercase().trim() == "y" {
            guessNumber();
        } else if encore.to_lowercase().trim() == "n" {
            break;
        }
    }
}
