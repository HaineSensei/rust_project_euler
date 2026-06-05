use super::ProblemStatus;
pub const STATUS: ProblemStatus = ProblemStatus::Complete;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

fn get_pentagonal(idx:usize, pentagonals: &mut HashMap<usize,usize>) -> usize {
    match pentagonals.get(&idx) {
        Some(pent) => *pent,
        None => {
            let pent = pentagonal(idx);
            pentagonals.insert(idx,pent);
            pent
        },
    }
}

fn pentagonal_index(n:usize, pentagonals: &mut HashMap<usize,usize>) -> Option<usize> {
    // n = x(3x - 1)/2
    // a = 36x^2 - 12x + 1 -> (6x-1)^2
    let a = 24*n + 1;
    
    // b = floor(6x - 1)
    let b = a.isqrt();
    if b*b != a {
        return None;
    }
    let c = (b+1)/6;
    if c*6 - 1 != b {
        None
    } else {
        pentagonals.insert(c,n);
        Some(c)
    }
}

fn pentagonal(n:usize) -> usize {
    if n == 0 {
        0
    } else {
        (n*(3*n - 1))/2
    }
}

fn next_upper_index(lower_idx: usize, index_cache: &mut HashMap<usize,usize>) -> usize {
    index_cache.get(&lower_idx).copied().unwrap_or_else(||lower_idx+1)
}

pub fn main() {
    let file = File::create("pairs_original.txt").unwrap();
    let mut writer = BufWriter::new(file);
    let mut pentagonals = HashMap::new();
    let mut out = 0;
    let mut index_cache = HashMap::new();
    'difference: for difference_idx in 1.. {
        let difference_target = get_pentagonal(difference_idx, &mut pentagonals);
        'lower: for lower_idx in 1.. {
            'upper: for upper_idx in next_upper_index(lower_idx,&mut index_cache).. {
                writeln!(writer,"{lower_idx},{upper_idx}");
                let lower = get_pentagonal(lower_idx, &mut pentagonals);
                let upper = get_pentagonal(upper_idx, &mut pentagonals);
                let sum = lower+upper;
                let diff = upper-lower;
                if diff >= difference_target {
                    if diff == difference_target && pentagonal_index(sum, &mut pentagonals).is_some() {
                        out = diff;
                        break 'difference;
                    }
                    index_cache.insert(lower_idx,upper_idx);
                    if lower_idx + 1 == upper_idx {
                        break 'lower;
                    }
                    break 'upper;
                }
            }
        }
    }
    println!("{out}");
}


#[cfg(test)]
mod tests {
    use super::*;

    
    fn is_pentagonal(n: usize) -> bool {
        // n = x(3x-1)/2 => 24n+1 = (6x-1)^2
        let a = 24 * n + 1;
        let b = a.isqrt();
        b * b == a && (b + 1) % 6 == 0
    }

    #[test]
    fn test_alt_main() {
        let file = File::create("pairs.txt").unwrap();
        let mut writer = BufWriter::new(file);
        
        let mut best_d = usize::MAX;
        
        'outer: for k in 2.. {
            let pk = pentagonal(k);
            if pk - pentagonal(k-1) > best_d {
                break;
            }
            for j in (1..k).rev() {
                writeln!(writer,"{j},{k}");
                let pj = pentagonal(j);
                let diff = pk - pj;
                if diff > best_d {
                    break;
                }
                let sum = pk + pj;
                if is_pentagonal(diff) && is_pentagonal(sum) {
                    best_d = best_d.min(diff);
                }
            }
        }
        println!("{best_d}");
    }
}