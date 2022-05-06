use std::{
    cmp::Ordering,
    env::{args, Args},
    io,
};

mod guessing_game;

fn main() {
    let mut args: Args = args();

    println!("{:?}", args);

    let programs = ["Guessing game"];

    let program_arg = args.nth(1);

    let program_index = choose_program(programs, program_arg);

    match program_index {
        0 => guessing_game::main(),
        _ => guessing_game::main(),
    }
}

// Prompts input from the user, processes it, and returns the selected program index.
fn choose_program(programs: [&str; 1], program_arg: Option<String>) -> usize {
    let mut program_arg_input = if let Some(..) = program_arg {
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
fn print_instructions(programs: [&str; 1]) {
    println!("Please select a program:");

    let mut i = 0;
    loop {
        println!("{}: {}", i, programs[i]);
        i += 1;
        if i == programs.len() {
            break;
        }
    }
}

// Resets the input argument to start over if the program does not exist.
fn reset_input_arg() -> String {
    println!("The program does not exist.");
    String::new()
}

// Prints selected program and returns the program index.
fn select_program(programs: [&str; 1], program_index: usize) -> usize {
    let program = programs[program_index];
    println!("You selected: {}", program);
    program_index
}
