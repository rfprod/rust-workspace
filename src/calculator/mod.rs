use colored::Colorize;
use meval::eval_str;
use std::{env::args, io};

/// The entry point of the program.
pub fn main() {
    Calculator::new();
}

/// Input arguments of the program.
struct InuputArguments {
    expression: Option<String>,
}

struct Calculator;

impl Calculator {
    /// Program constructor.
    fn new() -> Calculator {
        let mut program = Calculator;
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "Calculator initialized.".blue().bold());

        let args = self.args();

        self.calculate(args.expression);
    }

    /// Parses arguments passed to the program.
    fn args(&mut self) -> InuputArguments {
        let arguments: Vec<String> = args().collect();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), arguments);

        InuputArguments {
            expression: arguments.get(2).cloned(),
        }
    }

    /// The calculator program expresion parsing and evaluation logic.
    fn calculate(&mut self, expression_arg: Option<String>) {
        let is_some = expression_arg.is_some();
        let mut expression_arg_input = if is_some {
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
                    "Please input an expression without spaces:".yellow().bold()
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

            println!("\n{}: {}", "Result".green().bold(), result);

            expression_arg_input = String::new();
        }
    }
}
