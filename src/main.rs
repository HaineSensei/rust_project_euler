use std::io::{self, Write};

mod problems;
pub mod utils;

use problems::{PROBLEMS, ProblemStatus};

fn main() {
    println!("Project Euler Solutions");
    println!("----------------------");
    
    // Categorize problems by status
    let mut complete: Vec<usize> = Vec::new();
    let mut inefficient: Vec<usize> = Vec::new();
    let mut incomplete: Vec<usize> = Vec::new();
    
    for (&num, info) in PROBLEMS.iter() {
        match info.status {
            ProblemStatus::Complete => complete.push(num),
            ProblemStatus::Inefficient => inefficient.push(num),
            ProblemStatus::Incomplete => incomplete.push(num),
        }
    }
    
    complete.sort();
    inefficient.sort();
    incomplete.sort();
    
    println!("✓ Complete: {}", utils::ui_display::formatted_vec(&complete));
    println!("~ Inefficient: {}", utils::ui_display::formatted_vec(&inefficient));
    println!("✗ Incomplete: {}", utils::ui_display::formatted_vec(&incomplete));
    println!();

    loop {
        print!("Enter problem number (or 'q' to quit): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }
        
        match input.parse::<usize>() {
            Ok(num) => {
                if let Some(info) = PROBLEMS.get(&num) {
                    println!("\nRunning solution for Problem {} (Status: {:?}):", num, info.status);
                    (info.solver)();
                } else {
                    println!("Problem {} has not been implemented yet.", num);
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number or 'q' to quit.");
            }
        }
    }
}