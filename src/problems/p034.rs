// suppose ABC...D = A! + B! + ... + D!,
// then since A!, ..., D! <= 9!,
// ABC...D <= 9!*len(ABC...D) <= 9!*log_10(ABC...D)+1
// x <= 9!*log(x) + 1
// WolframAlpha says that this is equivalent to [1, ~2.30917×10^6] in interval notation,
// so checking to 10^7 will be sufficient.

const fn factorial(x:&u8) -> u64 {
    let x = *x;
    let mut out = 1;
    let mut i = 2;
    while i <= x {
        out*=i as u64;
        i+=1;
    }
    out
}

const FACT: [u64;10] = [
    factorial(&0),
    factorial(&1),
    factorial(&2),
    factorial(&3),
    factorial(&4),
    factorial(&5),
    factorial(&6),
    factorial(&7),
    factorial(&8),
    factorial(&9),
];

fn digits(x: u64) -> Vec<u8> {
    format!("{x}")
        .chars()
        .map(|c|{
            c.to_string().parse().unwrap()
        })
        .collect()
}

pub fn main() {
    let out: u64 = (10..10000000).filter(|&x| x == digits(x).iter().map(|&x|FACT[x as usize]).sum()).sum();
    println!("{out}");
}
