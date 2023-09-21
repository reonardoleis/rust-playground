use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut win = false;

    println!("Guess the number");

    while !win {
        println!("Input your number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess
            .trim()
            .parse::<u32>()
            .expect("Input was not a valid number. Try again!");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                win = true; 
                println!("Correct!")
            },
        }
    }
}
