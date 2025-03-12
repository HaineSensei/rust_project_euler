use std::{cmp, fs, sync::LazyLock};

static PYRAMID: LazyLock<Vec<Vec<u32>>> = LazyLock::new(|| {
    let text = fs::read_to_string("src/problems/files/p067_pyramid.txt").unwrap();
    let rows = text.split('\n');
    rows.map(|x| x.split(' ')
            .map(|x|x.parse().unwrap())
            .collect())
        .collect()
});

fn pyramid_get(x: usize, y: usize) -> u32 {
    *PYRAMID.get(x).unwrap().get(y).unwrap()
}

pub fn main() {
    let mut max_pyramid: Vec<Vec<u32>> = vec![vec![pyramid_get(0,0)]];
    for n in 1..PYRAMID.len() {
        let mut row: Vec<u32> = Vec::with_capacity(n);
        for i in 0..=n {
            let value = pyramid_get(n,i);
            if 0==i {
                let pyramid_row = max_pyramid.get(n-1).unwrap();
                row.push(value + pyramid_row.get(0).unwrap());
            } else if i==n {
                let pyramid_row = max_pyramid.get(n-1).unwrap();
                row.push(value + pyramid_row.get(n-1).unwrap());
            } else {
                let pyramid_row = max_pyramid.get(n-1).unwrap();
                row.push(value + cmp::max(pyramid_row.get(i-1).unwrap(),pyramid_row.get(i).unwrap()));
            }
        }
        max_pyramid.push(row);
    }
    let final_row = max_pyramid.get(max_pyramid.len()-1).unwrap();
    println!("{}",final_row.iter().max().unwrap());
}
