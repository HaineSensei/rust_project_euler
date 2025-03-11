pub fn main() {
    let mut product: u32 = 1;
    for n in 2..=20 {
        product *= n/gcd(n,product);
    }
    println!("{product}");
}

fn gcd(x:u32,y:u32) -> u32 {
    let mut x = x;
    let mut y = y;
    while x!=0 {
        (x,y) = (y%x,x);
    }
    y
}