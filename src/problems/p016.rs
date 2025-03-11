use num_bigint::BigUint;

pub fn main() {
    let two: BigUint = 2u8.into();
    let mut curr: BigUint = 1u8.into();
    for _ in 0..1000 {
        curr *= &two;
    }
    let two_power: BigUint = curr;

    let mut sum = 0;
    for x in format!("{}",two_power).chars() {
        if let Some(x) = x.to_digit(10) {
            sum+=x;
        }
    }
    println!("{}",sum);
}
