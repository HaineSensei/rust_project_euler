use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <problem_number> [status]", args[0]);
        println!("Status options: complete, inefficient, incomplete (default)");
        return;
    }

    let problem_num = args[1].parse::<usize>().expect("Invalid problem number");
    let status = args.get(2).map(|s| s.as_str()).unwrap_or("incomplete");

    if !["complete", "inefficient", "incomplete"].contains(&status) {
        println!("Invalid status. Use: complete, inefficient, or incomplete");
        return;
    }

    let status_variant = match status {
        "complete" => "Complete",
        "inefficient" => "Inefficient",
        _ => "Incomplete",
    };

    let filename = format!("src/problems/p{:03}.rs", problem_num);
    let path = Path::new(&filename);

    if path.exists() {
        update_status(path, status_variant);
        println!("Updated problem {} status to '{}'", problem_num, status);
    } else {
        create_problem_file(path, problem_num, status_variant);
        println!("Created problem {} with status '{}'", problem_num, status);
    }

    let exit = Command::new("cargo")
        .args(["build"])
        .status()
        .expect("Failed to run cargo build");

    if !exit.success() {
        println!("cargo build failed");
    }
}

fn create_problem_file(path: &Path, num: usize, status_variant: &str) {
    let mut file = fs::File::create(path).expect("Failed to create file");
    writeln!(file, "use super::ProblemStatus;").unwrap();
    writeln!(file, "pub const STATUS: ProblemStatus = ProblemStatus::{status_variant};").unwrap();
    writeln!(file).unwrap();
    writeln!(file, "pub fn main() {{").unwrap();
    writeln!(file, "    println!(\"Problem {num} not yet implemented\");").unwrap();
    writeln!(file, "}}").unwrap();
}

fn update_status(path: &Path, status_variant: &str) {
    let content = fs::read_to_string(path).expect("Failed to read file");
    let updated = content
        .lines()
        .map(|line| {
            if line.starts_with("pub const STATUS:") {
                format!("pub const STATUS: ProblemStatus = ProblemStatus::{status_variant};")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(path, updated).expect("Failed to write file");
}
