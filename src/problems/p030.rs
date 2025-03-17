use num_traits::pow;

/// Firstly, we need to identify an upper bound for all numbers which could satisfy this property.
/// Setting len : $\mathbb{N}$ -> $\mathbb{N}$ to be the length of decimal expansion function,
/// and val_n : $\mathbb{N}$ -> $\mathbb{N}$ to be the sum of nth powers of digits function.
/// Then, we know that
/// 
/// $$N = val_n(N) < len(N)*9^n$$
/// 
/// So,
/// 
/// $$N/len(N) < 9^n$$
/// 
/// since N $\mapsto$ N/len(N) is a piecewise increasing function with pieces being the constant
/// length intervals, and that each piece starts higher than the previous, we may find a sufficient
/// condition for this by looking for the first N in each piece and comparing N/len(N) with 9^n.


fn max_num(exponent: usize) -> usize {
    let mut count = 1;
    let mut curr = 1;
    while curr/count < pow(9, exponent) {
        curr *= 10;
        count += 1;
    }
    curr
}

fn val(num: usize, n: usize) -> usize {
    format!("{num}")
        .chars()
        .map(|c|{
            pow(c.to_digit(10).unwrap() as usize,n)
        })
        .sum()
}

pub fn main() {
    let n = 5;
    let total: usize = (10..max_num(n))
        .filter(|&num| num == val(num, n))
        .sum();
    println!("{total}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_4() {
        let n = 4;
        let total: usize = (10..max_num(n))
            .filter(|&num| num == val(num, n))
            .sum();
        assert_eq!(total,19316);
    }
}
