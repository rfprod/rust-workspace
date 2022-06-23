use colored::Colorize;
use meval::eval_str;
use std::{
    env::{args, Args},
    io,
};

// The program entry point.
pub fn main() {
    Calculator::new();
}

struct Calculator;

// The calculator implementation.
impl Calculator {
    // Creates a new calculator.
    fn new() -> Calculator {
        let mut program = Calculator;
        program.init();
        program
    }

    // Initializes the calculator.
    fn init(&mut self) {
        println!("\n{}", "Calculator initialized.".blue());

        let expression_arg = self.get_expression_arg();

        self.calculate(expression_arg);
    }

    // Parses the user expression.
    fn get_expression_arg(&mut self) -> Option<String> {
        let mut args: Args = args();

        println!("\n{}:\n{:?}", "Arguments".cyan(), args);

        args.nth(2)
    }

    // The calculator program expresion parsing and evaluation logic.
    fn calculate(&mut self, expression_arg: Option<String>) {
        let mut expression_arg_input = if let Some(..) = expression_arg {
            match expression_arg.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        loop {
            let mut expression_input = String::new();

            if expression_arg_input.trim().is_empty() && expression_input.trim().is_empty() {
                println!(
                    "\n{}",
                    "Please input an expression without spaces:".yellow()
                );

                io::stdin()
                    .read_line(&mut expression_input)
                    .expect("Failed to read line");
            } else if expression_input.trim().is_empty() {
                expression_input = expression_arg_input.to_string();
            }

            if expression_input.trim().is_empty() {
                expression_arg_input = String::new();
                continue;
            }

            let expression = expression_input.as_str().trim().to_string();

            println!("\n{}: {}", "Your expression".cyan(), expression);

            let result = eval_str(expression.as_str()).unwrap();

            println!("\n{}: {}", "Result".green(), result);

            expression_arg_input = String::new();
        }
    }
}
