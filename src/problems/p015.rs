fn binomial(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    
    let k: u64 = std::cmp::min(k, n - k);
    let mut c: u64 = 1;
    
    for i in 0..k {
        c = c * (n - i) / (i + 1);
    }
    
    c
}

pub fn main() {
    println!("{}",binomial(40,20));
}
