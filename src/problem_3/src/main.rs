fn main() {
    let mut number: u64 = 600851475143;
    let mut n: u64 = 2;
    let mut max: u64 = 1;
    while number != 1 {
        while number % n == 0 {
            number /= n;
            max = n;
            println!("{n}");
        }
        n+=1;
    }
    println!("{max}");
}