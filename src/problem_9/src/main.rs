fn main() {
    let triples = (1..=1000).flat_map(|x| (1..=1000-x).map(move |y| (x,y,1000-x-y)))
                            .filter(|x| x.0*x.0 - x.1*x.1 - x.2*x.2 == 0);
    for x in triples {
        println!("{}",x.0*x.1*x.2);
    }
}
