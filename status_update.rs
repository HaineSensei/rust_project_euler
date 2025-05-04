use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <problem_number> <status>", args[0]);
        println!("Status options: complete, inefficient, incomplete");
        return;
    }
    
    let problem_num = args[1].parse::<usize>().expect("Invalid problem number");
    let new_status = &args[2];
    
    if !["complete", "inefficient", "incomplete"].contains(&new_status.as_str()) {
        println!("Invalid status. Use: complete, inefficient, or incomplete");
        return;
    }
    
    update_status_in_mod_file(problem_num, new_status);
    println!("Updated problem {} status to '{}'", problem_num, new_status);
}

fn update_status_in_mod_file(num: usize, new_status: &str) {
    let mod_path = Path::new("src/problems/mod.rs");
    let content = fs::read_to_string(mod_path).expect("Failed to read mod.rs");
    
    let status_enum = match new_status {
        "complete" => "ProblemStatus::Complete",
        "inefficient" => "ProblemStatus::Inefficient",
        _ => "ProblemStatus::Incomplete",
    };
    
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut found = false;
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains(&format!("problems.insert({},", num)) {
            lines[i] = format!("    problems.insert({}, ProblemInfo {{ solver: p{:03}::main, status: {} }});", 
                              num, num, status_enum);
            found = true;
            break;
        }
    }
    
    if !found {
        println!("Warning: Problem {} not found in mod.rs", num);
        return;
    }
    
    fs::write(mod_path, lines.join("\n")).expect("Failed to write mod.rs");
}