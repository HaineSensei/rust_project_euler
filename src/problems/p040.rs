
fn num_block_length(num_digits:usize) -> usize {
    9*10usize.pow((num_digits-1) as u32)*num_digits
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DigitBlock {
    start_index: usize,
    num_digits: usize
}

// number of digits in the number in the block the nth digit is in.
fn digit_block(digit_index:usize) -> DigitBlock {
    let mut i = 1;
    let mut total = num_block_length(i);
    while digit_index > total {
        i+=1;
        total+=num_block_length(i);
    }
    DigitBlock {
        start_index: total-num_block_length(i),
        num_digits: i
    }
}

// d_n
fn d(n:usize)->usize {
    let digit_block = digit_block(n);

    let position_in_block = n - digit_block.start_index;
    let num_in_block = (position_in_block-1)/digit_block.num_digits;
    let digit_in_num = (position_in_block-1)%digit_block.num_digits;
    let first_num = 10usize.pow((digit_block.num_digits-1) as u32);
    let num = first_num+num_in_block;
    let num_str = format!("{num}");
    num_str[digit_in_num..=digit_in_num].parse().unwrap()
}

pub fn main() {
    println!("{}",d(1)*d(10)*d(100)*d(1000)*d(10000)*d(100000)*d(1000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_block() {
        assert_eq!(digit_block(9),DigitBlock{start_index:0,num_digits:1});
        assert_eq!(digit_block(10),DigitBlock{start_index:9,num_digits:2});
    }

    #[test]
    fn test_d() {
        let digits: String = (0..).flat_map(|x|format!("{x}").chars().collect::<Vec<_>>().into_iter()).take(1000).collect();
        for i in 1..1000 {
            assert_eq!(digits.chars().nth(i),format!("{}",d(i)).chars().next(),"failed with {i}");
        }
    }
}
