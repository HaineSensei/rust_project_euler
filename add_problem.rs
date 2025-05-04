use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <problem_number> [status]", args[0]);
        println!("Status options: complete, inefficient, incomplete (default)");
        return;
    }
    
    let problem_num = args[1].parse::<usize>().expect("Invalid problem number");
    let status = args.get(2).map(|s| s.as_str()).unwrap_or("incomplete");
    
    // Validate status
    if !["complete", "inefficient", "incomplete"].contains(&status) {
        println!("Invalid status. Use: complete, inefficient, or incomplete");
        return;
    }
    
    // Create problem file
    create_problem_file(problem_num);
    
    // Update mod.rs
    update_mod_file(problem_num, status);
    
    println!("Added problem {} with status '{}'", problem_num, status);
}

fn create_problem_file(num: usize) {
    let filename = format!("src/problems/p{:03}.rs", num);
    let path = Path::new(&filename);
    
    if path.exists() {
        println!("Warning: {} already exists", filename);
        return;
    }
    
    let mut file = File::create(path).expect("Failed to create file");
    writeln!(file, "pub fn main() {{").unwrap();
    writeln!(file, "    // TODO: Implement solution for problem {}", num).unwrap();
    writeln!(file, "    println!(\"Problem {} not yet implemented\");", num).unwrap();
    writeln!(file, "}}").unwrap();
}

fn update_mod_file(num: usize, status: &str) {
    let mod_path = Path::new("src/problems/mod.rs");
    let content = fs::read_to_string(mod_path).expect("Failed to read mod.rs");
    
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    
    // Find where to insert the module declaration
    let module_name = format!("pub mod p{:03};", num);
    let mut module_inserted = false;
    let mut module_insert_index = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains("pub mod p") {
            if line == &module_name {
                module_inserted = true;
                break;
            }
            // Extract number from module declaration
            if let Some(start) = line.find("pub mod p") {
                if let Some(end) = line.find(';') {
                    if let Ok(existing_num) = line[start+9..end].parse::<usize>() {
                        if existing_num > num {
                            module_insert_index = i;
                            break;
                        }
                    }
                }
            }
            module_insert_index = i + 1;
        }
    }
    
    if !module_inserted {
        lines.insert(module_insert_index, module_name);
    }
    
    // Find where to insert the problem entry
    let status_enum = match status {
        "complete" => "ProblemStatus::Complete",
        "inefficient" => "ProblemStatus::Inefficient",
        _ => "ProblemStatus::Incomplete",
    };
    
    let mut problem_inserted = false;
    let mut problem_insert_index = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains("problems.insert(") {
            // Extract number from problem entry
            if let Some(start) = line.find("problems.insert(") {
                if let Some(comma) = line[start..].find(',') {
                    if let Ok(existing_num) = line[start+16..start+comma].trim().parse::<usize>() {
                        if existing_num == num {
                            // Update existing entry
                            lines[i] = format!("    problems.insert({}, ProblemInfo {{ solver: p{:03}::main, status: {} }});", 
                                num, num, status_enum);
                            problem_inserted = true;
                            break;
                        } else if existing_num > num {
                            problem_insert_index = i;
                            break;
                        }
                    }
                }
            }
            problem_insert_index = i + 1;
        }
    }
    
    if !problem_inserted {
        lines.insert(problem_insert_index, format!("    problems.insert({}, ProblemInfo {{ solver: p{:03}::main, status: {} }});", 
            num, num, status_enum));
    }
    
    // Write back to file
    fs::write(mod_path, lines.join("\n")).expect("Failed to write mod.rs");
}