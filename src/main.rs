use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut range = thread_rng();

    let secret_number: i32 = range.gen_range(1..2);

    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        let guess = match guess_input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess_input);

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
