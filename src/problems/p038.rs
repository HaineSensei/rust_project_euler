

use std::collections::HashSet;

fn unique(s:&str)->bool {
    s.chars().collect::<HashSet<_>>().len() == s.len()
}

fn pandigital(s:&str)->bool {
    let chars = s.chars().collect::<HashSet<_>>();
    HashSet::from(['1','2','3','4','5','6','7','8','9']) == chars
}

fn pandigital_product(base_number: usize) -> Option<usize> {
    let mut i = 1;
    let mut pandigital_out = String::new();
    loop {
        pandigital_out.push_str(&format!("{}",i*base_number));
        if !unique(&pandigital_out) {
            return None;
        }
        if pandigital(&pandigital_out) {
            return Some(pandigital_out.parse().unwrap());
        }
        i+=1;
    }
}

pub fn main() {
    // since n > 1, we must have our base number <= 9876
    println!("{}",
        (1..=9876)
        .filter_map(|x|pandigital_product(x))
        .max()
        .unwrap()
    );

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        assert!(unique(""));
        assert!(unique("1"));
        assert!(!unique("11"));
    }

    #[test]
    fn test_pandigital_product() {
        assert_eq!(pandigital_product(1),Some(123456789));
        assert_eq!(pandigital_product(2),None);
        assert_eq!(pandigital_product(192),Some(192384576));
        assert_eq!(pandigital_product(9),Some(918273645));
    }
}