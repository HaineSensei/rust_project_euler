use num_bigint::BigUint;

pub fn main() {
    let mut curr: BigUint = 1u8.into();
    for i in 1..100u8 {
        curr *= i;
    }
    let factorial_val: BigUint = curr;

    let mut sum = 0;
    for x in format!("{}", factorial_val).chars() {
        if let Some(x) = x.to_digit(10) {
            sum+=x;
        }
    }
    println!("{}",sum);
}
