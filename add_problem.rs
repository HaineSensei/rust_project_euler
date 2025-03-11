use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    // Get problem number from command line args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <problem_number>", args[0]);
        process::exit(1);
    }

    let problem_num = match args[1].parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Problem number must be a positive integer");
            process::exit(1);
        }
    };

    // Format problem number with leading zeros
    let problem_file = if problem_num < 10 {
        format!("p00{}", problem_num)
    } else if problem_num < 100 {
        format!("p0{}", problem_num)
    } else {
        format!("p{}", problem_num)
    };

    // Check if problem file already exists
    let problem_path = Path::new("src/problems").join(format!("{}.rs", problem_file));
    if problem_path.exists() {
        eprintln!("Error: Problem {} already exists at {:?}", problem_num, problem_path);
        process::exit(1);
    }

    // Create problem file with template
    println!("Creating problem file: {:?}", problem_path);
    fs::write(&problem_path, "pub fn main() {\n    \n}\n")?;

    // Update mod.rs
    let mod_path = Path::new("src/problems/mod.rs");
    let mod_content = fs::read_to_string(mod_path)?;
    let mod_line = format!("pub mod {};", problem_file);
    
    // Check if the module is already declared
    if !mod_content.contains(&mod_line) {
        println!("Updating mod.rs");
        let mut file = OpenOptions::new().append(true).open(mod_path)?;
        writeln!(file, "{}", mod_line)?;
    }

    // Update main.rs
    let main_path = Path::new("src/main.rs");
    let main_content = fs::read_to_string(main_path)?;
    
    // Find where to insert the new problem in main.rs
    let insert_line = format!("    probs.insert({},problems::{}::main);", problem_num, problem_file);
    
    if main_content.contains(&insert_line) {
        println!("Problem {} is already registered in main.rs", problem_num);
    } else {
        println!("Updating main.rs");
        
        // Find the right position to insert the new line
        let lines: Vec<&str> = main_content.lines().collect();
        let mut new_content = String::new();
        let mut inserted = false;
        
        // Find the last probs.insert line
        let mut last_insert_line_idx = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.trim().starts_with("probs.insert") {
                last_insert_line_idx = i;
            }
        }
        
        // Insert our new line after the last probs.insert line
        for (i, line) in lines.iter().enumerate() {
            new_content.push_str(line);
            new_content.push('\n');
            
            if i == last_insert_line_idx && !inserted {
                new_content.push_str(&insert_line);
                new_content.push('\n');
                inserted = true;
            }
        }
        
        // Handle case where we didn't find a place to insert
        if !inserted {
            eprintln!("Could not determine where to insert the new problem in main.rs");
            eprintln!("Please manually add: {}", insert_line);
        } else {
            // Remove trailing newline if there was one
            if new_content.ends_with('\n') {
                new_content.pop();
            }
            
            // Write the modified content back to main.rs
            fs::write(main_path, new_content)?;
        }
    }

    println!("Successfully added Problem {}!", problem_num);
    println!("Edit your solution at: {:?}", problem_path);
    
    Ok(())
}