use std::{collections::HashMap, hash::Hash};

// pythag triple with a < b < c
// 1 <= a <= 333, a < b <= (1000 - a)/2, b < c <= 499

fn max_freq<T: Hash+Eq+Copy, I: Iterator<Item = T>>(iter: I) -> Option<T> {
    let mut frequencies: HashMap<T,usize> = HashMap::new();
    for t in iter {
        match frequencies.get_mut(&t) {
            Some(x) => {
                *x+=1
            },
            None => {
                frequencies.insert(t,1);
            },
        }
    }
    frequencies.iter().max_by_key(|(_,freq)|*freq).map(|(t,_)|*t)
}

pub fn main() {
    let pythagoras_perimeters = (1..=333)
    .flat_map(
        |a:usize| 
        ((a+1)..=((1000-a)/2))
            .flat_map(move |b|
                ((b+1)..=499)
                    .map(move |c|(a,b,c))
            )
    )
    .filter_map(|(a,b,c)| if a*a + b*b == c*c {
        Some(a + b + c)
    } else {
        None
    });

    println!("{}",max_freq(pythagoras_perimeters).unwrap());
}
