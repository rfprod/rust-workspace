use colored::Colorize;
use std::{
    cmp::Ordering,
    env::{args, Args},
    io,
};
use sysinfo::{ProcessExt, System, SystemExt};

type Subprograms<'a> = [&'a str; 6];

// The system information program entry point.
pub fn main() {
    let mut args: Args = args();

    println!("\n{}:\n{:?}", "Arguments".cyan(), args);

    println!("\n{}", "Print system information.".blue());

    let subprogram_arg = args.nth(2);

    let subprogram_index = choose_subprogram(subprogram_arg);

    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    match subprogram_index {
        0 => print_system_information(system),
        1 => print_processes(system),
        2 => print_components_temperature(system),
        3 => print_disks_info(system),
        4 => print_memory_information(system),
        5 => print_all_information(system),
        _ => print_all_information(system),
    };
}

// Prompts input from the user, processes it, and returns the selected subprogram index.
fn choose_subprogram(subprogram_arg: Option<String>) -> usize {
    let mut subprogram_arg_input = if let Some(..) = subprogram_arg {
        match subprogram_arg.unwrap().trim().parse::<i32>() {
            Ok(value) => value.to_string(),
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let subprograms: Subprograms = [
        "System information",
        "Process information",
        "Components temperature",
        "Disks information",
        "Memory information",
        "All information",
    ];

    loop {
        let mut subprogram_input = String::new();

        if subprogram_arg_input.is_empty() {
            print_instructions(subprograms);

            io::stdin()
                .read_line(&mut subprogram_input)
                .expect("Failed to read line");
        } else {
            subprogram_input = subprogram_arg_input.to_string();
        }

        let subprogram_index = match subprogram_input.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match subprogram_index.cmp(&subprograms.len()) {
            Ordering::Less => {
                return select_subprogram(subprograms, subprogram_index);
            }
            Ordering::Greater => subprogram_arg_input = reset_input_arg(),
            Ordering::Equal => subprogram_arg_input = reset_input_arg(),
        }
    }
}

// Prints the subprogram selection instructions.
fn print_instructions(subprograms: Subprograms) {
    println!("\n{}", "Available subprograms:".cyan());

    let max_i = subprograms.len() - 1;
    let mut i = 0;
    while i <= max_i {
        println!("{}: {}", i, subprograms[i]);
        i += 1;
    }

    println!("\n{}, [0-{}]:", "Please select a subprogram".cyan(), max_i);
}

// Resets the input argument to start over if the program does not exist.
fn reset_input_arg() -> String {
    println!("\n{}", "The subprogram does not exist.".red());
    String::new()
}

// Prints selected subprogram and returns the program index.
fn select_subprogram(subprograms: Subprograms, subprogram_index: usize) -> usize {
    let subprogram = subprograms[subprogram_index];
    println!("You selected: {}", subprogram);
    subprogram_index
}

// Print information about the system.
fn print_system_information(system: System) -> System {
    println!("\n{}", "System information:".green());
    println!("System name:           {:?}", system.get_name());
    println!("System kernel version: {:?}", system.get_kernel_version());
    println!("System OS version:     {:?}", system.get_os_version());
    println!("System host name:      {:?}", system.get_host_name());
    system
}

// Print the system processes information.
fn print_processes(system: System) -> System {
    println!("\n{}", "System processes:".green());
    for (pid, proc_) in system.get_processes() {
        println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
    }
    system
}

// Print the temperature of the components.
fn print_components_temperature(system: System) -> System {
    println!("\n{}", "Components temperature:".green());
    for component in system.get_components() {
        println!("{:?}", component);
    }
    system
}

// Print the disks information.
fn print_disks_info(system: System) -> System {
    println!("\n{}", "Disks information:".green());
    for disk in system.get_disks() {
        println!("{:?}", disk);
    }
    system
}

// Print the RAM and SWAP information.
fn print_memory_information(system: System) -> System {
    println!("\n{}", "Memory information:".green());
    println!("Total memory: {} KB", system.get_total_memory());
    println!("Used memory : {} KB", system.get_used_memory());
    println!("Total swap  : {} KB", system.get_total_swap());
    println!("Used swap   : {} KB", system.get_used_swap());
    system
}

// Print all information about the system.
fn print_all_information(system: System) -> System {
    let mut sys = system;
    sys = print_system_information(sys);
    sys = print_processes(sys);
    sys = print_components_temperature(sys);
    sys = print_disks_info(sys);
    sys = print_memory_information(sys);
    sys
}
