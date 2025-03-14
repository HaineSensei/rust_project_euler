use num_bigint::BigUint;

pub fn main() {
    let mut first: BigUint = 0u8.into();
    let mut second: BigUint = 1u8.into();
    let mut index: usize = 1;
    while format!("{}", second).len() < 1000 {
        let temp = first + second.clone();
        (first,second,index) = (second,temp,index+1);
    }
    println!("{}",index);
}
