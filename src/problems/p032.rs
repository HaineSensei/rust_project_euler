use std::{collections::HashSet, fmt::Display, hash::Hash};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Hash)]
struct Digit(Option<u8>);

#[derive(Clone, Copy, Debug)]
struct Product([Digit; 11]);

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.vals()[2] == other.vals()[2]
    }
}

impl Eq for Product {}

impl Hash for Product {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.vals()[2].hash(state);
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut second = false;
        let mut out = String::new();
        for c in self.0 {
            match c.0 {
                Some(x) => {
                    out.push_str(format!("{x}").as_str());
                },
                None => {
                    if second {
                        out.push_str(" == ");
                    } else {
                        out.push_str(" * ");
                        second = true;
                    }
                }
            }
        }
        write!(f, "{out}")
    }
}

impl Product {
    fn reposition(x:usize) -> Option<usize> {
        match x {
            0 => Some(0),
            2 => Some(1),
            4 => Some(2),
            _ => None
        }
    }

    fn vals(self) -> [usize; 3] {
        let string = format!("{self}");
        let mut count = 0;
        let mut curr_start = 0;
        let mut out = [0,0,0];
        for (index,c) in string.chars().enumerate() {
            if c == ' ' {
                if let Some(x) = Self::reposition(count) {
                    out[x] = string[curr_start..index].parse().unwrap();
                }
                count += 1;
                curr_start = index + 1;
            }
        }
        out[2] = string[curr_start..].parse().unwrap();
        out
    }

    fn valid(self) -> bool {
        let [a, b, c] = self.vals();
        a * b == c
    }
}

const DIGITS : [u8; 9] = [1,2,3,4,5,6,7,8,9];

pub fn main() {
    let mut valids: HashSet<Product> = HashSet::new();
    for xs in DIGITS.iter().permutations(9) {
        for i in 1..10 {
            for j in i+2..10 {
                let mut product: [Digit; 11] = [Digit(None), Digit(None), Digit(None),
                                                Digit(None), Digit(None), Digit(None), 
                                                Digit(None), Digit(None), Digit(None), 
                                                Digit(None), Digit(None)];
                let mut count = 0;
                for &x in &xs {
                    if [i,j].contains(&count) {
                        count += 1;
                    }
                    product[count] = Digit(Some(*x));
                    count += 1;
                }
                let product = Product(product);
                if product.valid() {
                    valids.insert(product);
                }
            }
        }
    }
    assert!(valids.contains(&Product([
        Digit(Some(3)),
        Digit(Some(9)),
        Digit(None),
        Digit(Some(1)),
        Digit(Some(8)),
        Digit(Some(6)),
        Digit(None),
        Digit(Some(7)),
        Digit(Some(2)),
        Digit(Some(5)),
        Digit(Some(4))
    ])));

    println!("{}", valids.iter().map(|x| x.vals()[2]).sum::<usize>())
}
