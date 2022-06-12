use colored::Colorize;
use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering,
    env::{args, Args},
    io,
};

// The program entry point.
pub fn main() {
    let mut game = GuessingGame::new();
    game.init();
}

struct GuessingGame;

// The guessing game implementation.
impl GuessingGame {
    // Creates a new guessing game.
    fn new() -> GuessingGame {
        return GuessingGame;
    }

    // Initializes the guessing game.
    fn init(&mut self) {
        println!("\n{}", "Guessing game initialized.".blue());

        let secret_number: i32 = self.generate_secret();

        let guess_arg = self.get_guess_arg();

        self.start_guessing(secret_number, guess_arg);
    }

    // Parses the user guess.
    fn get_guess_arg(&mut self) -> Option<String> {
        let mut args: Args = args();

        println!("\n{}:\n{:?}", "Arguments".cyan(), args);

        let guess_arg = args.nth(2);

        guess_arg
    }

    // Generates a secret number.
    fn generate_secret(&mut self) -> i32 {
        let range_min = 1;
        let range_max = 101;

        println!(
            "\n{} [{}-{}]",
            "Guess the number between from range".cyan(),
            range_min,
            range_max
        );

        let mut range = thread_rng();

        let secret_number: i32 = range.gen_range(range_min..range_max);

        println!("{}: {}", "The secret number is".cyan(), secret_number);

        secret_number
    }

    // The main logic of the guessing game.
    fn start_guessing(&mut self, secret_number: i32, guess_arg: Option<String>) {
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
                println!("\n{}", "Please input your guess:".yellow());

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

            println!("\n{}: {}", "You guessed".cyan(), guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("{}", "Too small!".red());
                    self.precision(guess, secret_number);
                    guess_arg_input = String::new();
                }
                Ordering::Greater => {
                    println!("{}", "Too big!".red());
                    self.precision(guess, secret_number);
                    guess_arg_input = String::new();
                }
                Ordering::Equal => {
                    println!("{}", "You win!".green());
                    break;
                }
            }
        }
    }

    // Prints how far or close the user guess is.
    fn precision(&mut self, guess: i32, secret_number: i32) {
        let far_threshold = 10;
        let closer_threshold = far_threshold / 2;
        let closest_threshold = closer_threshold / 2;
        let absolute_difference = (secret_number - guess).abs();
        if absolute_difference < closest_threshold {
            println!("{}", "The guess is in the closest range.".green())
        } else if absolute_difference < closer_threshold {
            println!("{}", "The guess is closer.".bright_green())
        } else if absolute_difference < far_threshold {
            println!("{}", "The guess is far.".bright_red())
        } else {
            println!("{}", "The guess is too far.".red())
        }
    }
}
