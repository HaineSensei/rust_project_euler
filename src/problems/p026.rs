use std::collections::{HashMap};

fn recurrence(n: u16, x: u16) -> u16 {
    (x*10)%n
}

fn recurring_decimal_recip_len(n: u16) -> u16 {
    let mut seen: HashMap<u16,Option<u16>> = HashMap::new();
    seen.insert(0,None);
    seen.insert(1,Some(0));
    let mut count = 1;
    let mut curr = 10 % n;
    while let None = seen.get(&curr) {
        seen.insert(curr,Some(count));
        (curr, count) = (recurrence(n, curr), count+1);
    }
    match seen.get(&curr).unwrap() {
        Some(x) => count - x,
        None => 1,
    }
}

pub fn main() {
    let maximum = (1..1000).max_by(|&x, &y| {
        recurring_decimal_recip_len(x).cmp(&recurring_decimal_recip_len(y))
    }).unwrap();
    println!("{maximum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurring_decimal_recip_len() {
        assert_eq!(recurring_decimal_recip_len(7), 6);
        assert_eq!(recurring_decimal_recip_len(4), 1);
        assert_eq!(recurring_decimal_recip_len(6), 1);
        assert_eq!(recurring_decimal_recip_len(270), 3);
    }
}