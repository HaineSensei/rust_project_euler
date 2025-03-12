use std::{collections::HashMap, sync::LazyLock};

mod problems;
pub mod utils;

static PROBLEMS : LazyLock<HashMap<usize,fn() -> ()>> = LazyLock::new(|| {
    let mut probs : HashMap<usize, fn() -> ()> = HashMap::new();
    probs.insert(1,problems::p001::main);
    probs.insert(2,problems::p002::main);
    probs.insert(3,problems::p003::main);
    probs.insert(4,problems::p004::main);
    probs.insert(5,problems::p005::main);
    probs.insert(6,problems::p006::main);
    probs.insert(7,problems::p007::main);
    probs.insert(8,problems::p008::main);
    probs.insert(9,problems::p009::main);
    probs.insert(10,problems::p010::main);
    probs.insert(11,problems::p011::main);
    probs.insert(12,problems::p012::main);
    probs.insert(13,problems::p013::main);
    probs.insert(14,problems::p014::main);
    probs.insert(15,problems::p015::main);
    probs.insert(16,problems::p016::main);
    probs.insert(17,problems::p017::main);
    probs.insert(18,problems::p018::main);
    probs.insert(67,problems::p067::main);
    probs.insert(19,problems::p019::main);
    probs.insert(20,problems::p020::main);
    probs.insert(21,problems::p021::main);
    probs.insert(22,problems::p022::main);
    probs.insert(23,problems::p023::main);
    probs
});

// UI produced by Claude.ai since that was not the main purpose of this project.
fn main() {
    use std::io::{self, Write};
    
    println!("Project Euler Solutions");
    println!("----------------------");
    let mut prob_nums = PROBLEMS.keys().collect::<Vec<_>>();
    prob_nums.sort();
    println!("Available problems: {:?}", prob_nums);

    loop {
        print!("\nEnter problem number (or 'q' to quit): ");
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
                if let Some(solver) = PROBLEMS.get(&num) {
                    println!("\nRunning solution for Problem {}:", num);
                    solver();
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