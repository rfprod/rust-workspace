use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering,
    env::{args, Args},
    io,
};

fn main() {
    let mut args: Args = args();

    println!("{:?}", args);

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut range = thread_rng();

    let secret_number: i32 = range.gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    let guess_arg = args.nth(1);

    start_guessing(secret_number, guess_arg);
}

fn start_guessing(secret_number: i32, guess_arg: Option<String>) {
    let mut guess_arg_input = if let Some(..) = guess_arg {
        match guess_arg.unwrap().trim().parse::<i32>() {
            Ok(value) => value.to_string(),
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    loop {
        let mut guess_input = String::new();

        if guess_arg_input.is_empty() {
            io::stdin()
                .read_line(&mut guess_input)
                .expect("Failed to read line");
        } else {
            guess_input = guess_arg_input.to_string();
        }

        let guess = match guess_input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_arg_input = String::new()
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_arg_input = String::new()
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
