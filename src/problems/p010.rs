const MAX: u64 = 2000000;

pub fn main() {
    let mut prev: Vec<u64> = Vec::new();
    let mut curr: Vec<u64> = (2..MAX).collect();
    let mut p: u64;
    let mut total: u64 = 0;
    while prev != curr {
        ((curr,p),prev) = (next(&curr),curr);
        total += p;
        if p*p > MAX {
            break;
        }
    }
    println!("{}",total+curr.iter().sum::<u64>());
}

fn next(xs: &Vec<u64>) -> (Vec<u64>,u64) {
    let p = xs[0];
    (xs.iter().filter(|&&x| x % p != 0).map(|x| *x).collect(),p)
}