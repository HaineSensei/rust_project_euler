use std::fs::read_to_string;

use crate::utils::alphabetical_sum;

fn is_triangle(x:usize) -> bool {
    // x = n(n+1)/2

    // a = 4n^2 + 4n
    let a = 8*x;

    // b = 4n^2 + 4n + 1 = (2n + 1)^2
    let b = a + 1;

    // c = floor(2n + 1)
    let c = b.isqrt();
    if c*c != b {return false};
    // c = 2n + 1

    c%2==1 // implies n an integer
}

fn is_triangle_word(word:&str) -> bool {
    is_triangle(alphabetical_sum(word).unwrap())
}

pub fn main() {
    let in_string = read_to_string("src/problems/files/p042_words.txt").unwrap();
    let words = in_string[1..in_string.len()-1].split(r#"",""#);
    let out = {
        words
        .filter(|word|is_triangle_word(word))
        .count()
    };
    println!("{out}");
}
