pub fn main() {
    let mut max: u32 = 0;
    for n in 100..1000 {
        for m in 100..1000 {
            let mn = m*n;
            if mn>=max {
                if is_palindrome(mn) {
                    max = mn;
                }
            }
        }
    }
    println!("{max}")
}

fn is_palindrome(n: u32) -> bool {
    let n: String = n.to_string();
    for i in 0..n.len() {
        if n[i..i+1] != n[n.len()-i-1..n.len()-i] {
            return false
        }
    }
    true
}