pub fn main() {
    println!("old: {}",prime(10001));
    println!("new: {}", prime_new(10001));
    println!("old 1000001st: {}", prime(1000001));
}

fn prime(n:usize) -> u64 {
    let mut primes = Vec::<u64>::new();
    let mut current: u64 = 1;
    'outer: while primes.len() < n {
        current += 1;
        for p in primes.iter() {
            if p*p>current {
                break
            }
            if current%p==0 {
                continue 'outer;
            }
        }
        primes.push(current);
    }
    primes[n-1]
}

// this is very slow apparently, though once it's completed, it's quick.
fn prime_new(n: usize) -> u128 {
    let mut curr;
    curr = n as u128 * (n as f64).ln() as u128 * 2;
    let x;
    loop {
        if let Some(&i) = crate::utils::primes_less_than(curr).collect::<Vec<_>>().get(n-1) {
            x = i;
            break;
        }
        curr*=2;
    }
    x
}

