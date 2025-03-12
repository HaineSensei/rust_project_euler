fn amicable(x:u32) -> bool {
    d(d(x)) == x && d(x) != x
}

fn d(x: u32) -> u32 {
    let mut out = 0;
    for i in 1..x {
        if x%i == 0 {
            out += i;
        }
    }
    out
}

pub fn main() {
    let sum : u32 = (2..10000)
        .filter(|&x| amicable(x))
        .sum();
    println!("{sum}");
}

