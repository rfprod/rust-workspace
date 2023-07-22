use colored::Colorize;
use std::{
    cmp::Ordering,
    env::{args, Args},
    io,
};

mod calculator;
mod guessing_game;
mod linfa_train;
mod open_weather;
mod system_information;

type Programs<'a> = [&'a str; 5];

// The main program entry point.
fn main() {
    let mut args: Args = args();

    println!("\n{}:\n{:?}", "Arguments".cyan().bold(), args);

    let programs: Programs = [
        "Guessing game",
        "Open Weather",
        "System information",
        "Calculator",
        "Linfa train",
    ];

    let program_arg = args.nth(1);

    let program_index = choose_program(programs, program_arg);

    match program_index {
        0 => guessing_game::main(),
        1 => open_weather::main(),
        2 => system_information::main(),
        3 => calculator::main(),
        4 => linfa_train::main(),
        _ => guessing_game::main(),
    }
}

// Prompts input from the user, processes it, and returns the selected program index.
fn choose_program(programs: Programs, program_arg: Option<String>) -> usize {
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
            print_instructions(programs);

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
                return select_program(programs, program_index);
            }
            Ordering::Greater => program_arg_input = reset_input_arg(),
            Ordering::Equal => program_arg_input = reset_input_arg(),
        }
    }
}

// Prints the program selection instructions.
fn print_instructions(programs: Programs) {
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

// Resets the input argument to start over if the program does not exist.
fn reset_input_arg() -> String {
    println!("\n{}", "The subprogram does not exist.".red());
    String::new()
}

// Prints selected program and returns the program index.
fn select_program(programs: Programs, program_index: usize) -> usize {
    let program = programs[program_index];
    println!("You selected: {}", program);
    program_index
}
