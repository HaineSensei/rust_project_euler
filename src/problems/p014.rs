use std::{collections::HashMap, sync::{LazyLock, Mutex}};

pub static COLLATZ_LENGTHS: LazyLock<Mutex<HashMap<u64, u64>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(1, 1); // Base case
    Mutex::new(map)
});

fn collatz_length(x: u64) -> u64 {
    let mut known_lens = COLLATZ_LENGTHS.lock().unwrap();
    let mut curr = x;
    let mut unknown_lens: Vec<u64> = Vec::new();
    loop {
        let value = known_lens.get(&curr);
        if let Some(&len) = value {
            for (i, &val) in unknown_lens.iter().rev().enumerate() {
                known_lens.insert(val,len+1+i as u64);
            }
            break;
        } else {
            unknown_lens.push(curr);
            curr = collatz_next(curr);
        }
    }
    *known_lens.get(&x).unwrap()
}

pub fn main() {
    let mut max_val = 0;
    let mut max_len = 0;
    for i in 1..1000000 {
        let len = collatz_length(i);
        if len > max_len {
            max_val = i;
            max_len = len;
        }
    }
    println!("{}",max_val);
}

fn collatz_next(x:u64) -> u64 {
    if x%2==0 {
        x>>1
    } else {
        3*x + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collatz_length() {
        // This will use the global cache
        assert_eq!(collatz_length(1), 1);
        assert_eq!(collatz_length(2), 2);
        assert_eq!(collatz_length(3), 8);
        assert_eq!(collatz_length(4), 3);
    }
    
    #[test]
    fn test_collatz_next() {
        assert_eq!(collatz_next(1), 4);
        assert_eq!(collatz_next(2), 1);
        assert_eq!(collatz_next(3), 10);
        assert_eq!(collatz_next(4), 2);
    }
}
