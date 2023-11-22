//! Linfa train module.

use colored::Colorize;
use std::cmp::Ordering;
use std::env::args;
use std::io;

type LinfaTrainPrograms<'a> = [&'a str; 2];

mod decision_tree;
mod logistic_regression;

/// The entry point of the program.
pub fn main() {
    LinfaTrain::new();
}

/// Input arguments of the program.
struct InuputArguments {
    program_index: Option<String>,
}

struct LinfaTrain;

/// Source: https:///github.com/DataPsycho/data-pipelines-in-rust/blob/main/diabetes_ml_pipeline/Cargo.toml
impl LinfaTrain {
    /// Program constructor.
    fn new() -> LinfaTrain {
        let mut program = LinfaTrain;
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "Linfa train initialized.".blue().bold());

        let args = self.args();

        let programs: LinfaTrainPrograms = ["Logistic regression", "Decision tree"];

        let program_index = self.choose_program(programs, args.program_index);

        match program_index {
            0 => logistic_regression::main(),
            1 => decision_tree::main(),
            _ => {
                panic!("Program does not exist");
            }
        }
    }

    /// Parses arguments passed to the program.
    fn args(&mut self) -> InuputArguments {
        let arguments: Vec<String> = args().collect();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), arguments);

        let program_index = arguments.get(3).cloned();

        InuputArguments { program_index }
    }

    /// Prompts input from the user, processes it, and returns the index of the selected program.
    fn choose_program(&self, programs: LinfaTrainPrograms, program_arg: Option<String>) -> usize {
        let is_some = program_arg.is_some();
        let mut program_arg_input = if is_some {
            match program_arg.unwrap().trim().parse::<i32>() {
                Ok(value) => value.to_string(),
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        loop {
            let mut program_input = String::new();

            if program_arg_input.is_empty() {
                self.print_instructions(programs);

                io::stdin()
                    .read_line(&mut program_input)
                    .expect("Failed to read line");
            } else {
                program_input = program_arg_input.to_string();
            }

            let program_index = match program_input.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match program_index.cmp(&programs.len()) {
                Ordering::Less => {
                    return self.select_program(programs, program_index);
                }
                Ordering::Greater => program_arg_input = self.reset_input_arg(),
                Ordering::Equal => program_arg_input = self.reset_input_arg(),
            }
        }
    }

    /// Prints the program selection instructions.
    fn print_instructions(&self, programs: LinfaTrainPrograms) {
        println!("\n{}", "Available programs:".yellow().bold());

        let max_i = programs.len() - 1;
        let mut i = 0;
        while i <= max_i {
            println!("{}: {}", i, programs[i]);
            i += 1;
        }

        println!(
            "\n{}, [0-{}]:",
            "Please select a program".yellow().bold(),
            max_i
        );
    }

    /// Resets the input argument to start over if the program does not exist.
    fn reset_input_arg(&self) -> String {
        println!("\n{}", "The subprogram does not exist.".red());
        String::new()
    }

    /// Prints selected program and returns the program index.
    fn select_program(&self, programs: LinfaTrainPrograms, program_index: usize) -> usize {
        let program = programs[program_index];
        println!("You selected: {}", program);
        program_index
    }
}
