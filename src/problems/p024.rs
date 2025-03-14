use std::fmt::Display;

#[derive(Copy, Clone)]
struct Permutation10 {
    perm: [u8; 10]
}

impl Display for Permutation10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",
            self.perm[0],
            self.perm[1],
            self.perm[2],
            self.perm[3],
            self.perm[4],
            self.perm[5],
            self.perm[6],
            self.perm[7],
            self.perm[8],
            self.perm[9]
        )
    }
}

const DIGITS: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];

fn factorial(x: u8) -> u128 {
    let mut out = 1;
    let mut curr = 1;
    while curr<=x {
        out *= curr as u128;
        curr += 1;
    }
    out
}

pub fn main() {
    let mut remaining_digits: Vec<u8> = DIGITS.into();
    let mut curr_index = 1_000_000-1; // zero-indexing vs one-indexing...
    let mut indices: Vec<u8> = Vec::with_capacity(10);
    for i in (0u8..10).rev() {
        let fact = factorial(i);
        let next = remaining_digits[(curr_index/fact) as usize];
        indices.push(next); 
        remaining_digits = remaining_digits
            .iter()
            .filter(|&&x| x != next)
            .map(|&x| x)
            .collect();
        curr_index = curr_index%fact;
    }
    println!("{}",
        Permutation10 {
            perm: indices.try_into().unwrap()
        }
    )
}
