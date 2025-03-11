pub fn main() {
    println!("{}",(|x: i32| x*x)((1..=100).sum()) - (1..=100).map(|x: i32| x*x).sum::<i32>());
}
