use std::fs;

fn value(name: &str) -> u64 {
    name.chars()
        .map(|c|(c as u8 - ('A' as u8) + 1) as u64)
        .sum()
}

pub fn main() {
    let text = fs::read_to_string("src/problems/files/p022_names.txt").unwrap();
    let mut names : Vec<&str> = text.split(' ').collect();
    names.sort();
    // position in list is i + 1
    println!("{}", names
        .iter()
        .enumerate()
        .map(|(i,&name)| (i+1) as u64 * value(name) )
        .sum::<u64>()
    );
}
