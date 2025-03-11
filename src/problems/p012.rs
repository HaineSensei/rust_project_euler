use std::collections::HashMap;

const TARGET_TOTAL: usize = 500;

pub fn main() {
    assert_eq!(floor_sqrt(64),8);
    assert_eq!(floor_sqrt(80),8);
    assert_eq!(floor_sqrt(65),8);
    let mut primes: Vec<u64> = Vec::new();
    let mut curr: u128 = 3;
    let mut curr_triangle: u128 = 6;
    loop {
        curr+=1;
        curr_triangle+=curr;
        update(&mut primes,floor_sqrt(curr_triangle));
        let factors: HashMap<u64, usize> = factor(curr_triangle, &mut primes);
        let tau_curr_triangle: usize = factors.iter().map(|(_,&b)| b + 1).product::<usize>();
        if tau_curr_triangle >= TARGET_TOTAL {
            println!("{curr_triangle}");
            break;
        }
    }
}

fn update(primes: &mut Vec<u64>, n:u64) {
    let first = primes.iter().max();
    let mut curr = match first {
        Some(&n) => n,
        None => 1
    };
    while curr <= n {
        curr += 1;
        if all(primes.iter().map(|&p| curr%p != 0)) {
            primes.push(curr);
        }
    }
}

fn floor_sqrt(n: u128) -> u64 {
    if n == 0 {
        0
    } else {
        let mut upper: u64 = 0u64.wrapping_sub(1);
        let mut lower: u64 = 1;
        let mut mid: u64 = 2u64.pow(63);
        while upper - lower > 1 {
            let square = (mid as u128)*(mid as u128);
            if (square <= n) & ((upper as u128)*(upper as u128) < n) {
                return mid;
            } else if square < n {
                (upper,lower) = (upper,mid,);
                mid = match ((upper as u128 + mid as u128)/2).try_into().ok() {
                    Some(num) => num,
                    None => unreachable!(),
                };
            } else {
                (lower,upper) = (lower,mid);
                mid = match ((lower as u128 + mid as u128)/2).try_into().ok() {
                    Some(num) => num,
                    None => unreachable!(),
                };
            }
        }
        if (upper as u128)*(upper as u128) == n {
            upper
        } else {
            lower
        }
    }
}

fn all(xs: impl Iterator<Item = bool>) -> bool {
    for b in xs {
        if !b {
            return false;
        }
    }
    true
}

fn factor(n: u128, primes: &mut Vec<u64>) -> HashMap<u64, usize> {
    let mut num = n;
    let mut powers: HashMap<u64, usize> = HashMap::new();
    for &p in primes.iter() {
        while num%(p as u128)==0 {
            num /= p as u128;
            powers.entry(p)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    if num > 1 {
        match num.try_into().ok() {
            Some(num) => {
                powers.insert(num,1);
            },
            None => {
                // use 1 in place of num since num does not fit in u64 and cannot appear via any other means.
                powers.insert(1,1);
            }
        }
    }
    powers
}