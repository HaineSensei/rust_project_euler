fn is_palindrome(xs: &str) -> bool {
    xs == xs.chars().rev().collect::<String>()
}

fn is_double_palindrome(x: u64) -> bool {
    is_palindrome(&format!("{x}")) && is_palindrome(&format!("{x:0b}"))
}

pub fn main() {
    println!("{}",(1..1000000).filter(|&x| is_double_palindrome(x)).sum::<u64>())
}
