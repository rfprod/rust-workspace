//! System information module.

use colored::Colorize;
use std::{cmp::Ordering, env::args, io};
use sysinfo::{ProcessExt, System, SystemExt};

type Subprograms<'a> = [&'a str; 6];

/// The entry point of the program.
pub fn main() {
    SystemInformation::new();
}

struct InuputArguments {
    subprogram: Option<String>,
}

struct SystemInformation;

impl SystemInformation {
    /// Program constructor.
    fn new() -> SystemInformation {
        let mut program = SystemInformation;
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "System information initialized.".blue().bold());

        let args = self.args();

        let subprogram_index = self.choose_subprogram(args.subprogram);

        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        match subprogram_index {
            0 => self.print_system_information(system),
            1 => self.print_processes(system),
            2 => self.print_components_temperature(system),
            3 => self.print_disks_info(system),
            4 => self.print_memory_information(system),
            5 => self.print_all_information(system),
            _ => self.print_all_information(system),
        };
    }

    /// Parses arguments passed to the program.
    fn args(&mut self) -> InuputArguments {
        let arguments: Vec<String> = args().collect();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), arguments);

        InuputArguments {
            subprogram: arguments.get(2).cloned(),
        }
    }

    /// Prompts input from the user, processes it, and returns the selected subprogram index.
    fn choose_subprogram(&mut self, subprogram_arg: Option<String>) -> usize {
        let is_some = subprogram_arg.is_some();
        let mut subprogram_arg_input = if is_some {
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
                self.print_instructions(subprograms);

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
                    return self.select_subprogram(subprograms, subprogram_index);
                }
                Ordering::Greater => subprogram_arg_input = self.reset_input_arg(),
                Ordering::Equal => subprogram_arg_input = self.reset_input_arg(),
            }
        }
    }

    /// Prints the subprogram selection instructions.
    fn print_instructions(&mut self, subprograms: Subprograms) {
        println!("\n{}", "Available subprograms:".yellow().bold());

        let max_i = subprograms.len() - 1;
        let mut i = 0;
        while i <= max_i {
            println!("{}: {}", i, subprograms[i]);
            i += 1;
        }

        println!(
            "\n{}, [0-{}]:",
            "Please select a subprogram".yellow().bold(),
            max_i
        );
    }

    /// Resets the input argument to start over if the program does not exist.
    fn reset_input_arg(&mut self) -> String {
        println!("\n{}", "The subprogram does not exist.".red());
        String::new()
    }

    /// Prints selected subprogram and returns the program index.
    fn select_subprogram(&mut self, subprograms: Subprograms, subprogram_index: usize) -> usize {
        let subprogram = subprograms[subprogram_index];
        println!("You selected: {}", subprogram);
        subprogram_index
    }

    /// Print information about the system.
    fn print_system_information(&mut self, system: System) -> System {
        println!("\n{}", "System information:".green());
        println!("System name:           {:?}", system.get_name().unwrap());
        println!(
            "System kernel version: {:?}",
            system.get_kernel_version().unwrap()
        );
        println!(
            "System OS version:     {:?}",
            system.get_os_version().unwrap()
        );
        println!(
            "System host name:      {:?}",
            system.get_host_name().unwrap()
        );
        system
    }

    /// Print the system processes information.
    fn print_processes(&mut self, system: System) -> System {
        println!("\n{}", "System processes:".green());
        for (pid, proc_) in system.get_processes() {
            let double_indent = "\t\t";
            let indent = "\t";
            println!(
                "STATUS {}\t|\tPID {}{}|\tNAME {:?}",
                proc_.status().as_str(),
                pid,
                if pid.lt(&999) { double_indent } else { indent },
                proc_.name(),
            );
        }
        system
    }

    /// Print the temperature of the components.
    fn print_components_temperature(&mut self, system: System) -> System {
        println!("\n{}", "Components temperature:".green());
        for component in system.get_components() {
            println!("{:?}", component);
        }
        system
    }

    /// Print the disks information.
    fn print_disks_info(&mut self, system: System) -> System {
        println!("\n{}", "Disks information:".green());
        for disk in system.get_disks() {
            println!("{:?}", disk);
        }
        system
    }

    /// Print the RAM and SWAP information.
    fn print_memory_information(&mut self, system: System) -> System {
        println!("\n{}", "Memory information:".green());
        println!("Total memory: {} KB", system.get_total_memory());
        println!("Used memory : {} KB", system.get_used_memory());
        println!("Total swap  : {} KB", system.get_total_swap());
        println!("Used swap   : {} KB", system.get_used_swap());
        system
    }

    /// Print all information about the system.
    fn print_all_information(&mut self, system: System) -> System {
        let mut sys = system;
        sys = self.print_system_information(sys);
        sys = self.print_processes(sys);
        sys = self.print_components_temperature(sys);
        sys = self.print_disks_info(sys);
        sys = self.print_memory_information(sys);
        sys
    }
}
