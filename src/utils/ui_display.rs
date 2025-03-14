use std::fmt::Display;

#[derive(PartialEq, Debug)]
struct Range {
    start: usize,
    end: usize
}

#[derive(PartialEq, Debug)]
enum Part {
    Range(Range),
    Singleton(usize)
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Range(range) => write!(f, "{}-{}", range.start, range.end),
            Part::Singleton(x) => write!(f, "{}", *x),
        }
    }
}

fn vec_to_parts(xs: &Vec<usize>) -> Vec<Part> {
    if xs.len() == 0 {
        return Vec::new()
    } else if xs.len() == 1 {
        return vec![Part::Singleton(xs[0])];
    }
    let mut xs = xs.clone();
    xs.sort();
    let xs = xs;
    let mut start = None;
    let mut prev = None;
    let mut out = Vec::new();
    for &curr in &xs {
        match prev {
            None => {
                start = Some(curr);
            },
            Some(prev) => {
                if curr != prev+1 {
                    let start_val = start.expect("start should have been initialised by this point.");
                    if prev == start_val {
                        out.push(Part::Singleton(start_val));
                    } else {
                        out.push(Part::Range(Range { start: start_val, end: prev }));
                    }
                    start = Some(curr);
                }
            }
        }
        prev = Some(curr);
    }
    let len = xs.len();
    if let (Some(start), Some(&last)) = (start, xs.get(len-1)) {
        if last == start {
            out.push(Part::Singleton(last));
        } else {
            out.push(Part::Range(Range { start: start, end: last }));
        }
    }
    out
}

fn formatted_vec_of_parts(xs: &Vec<Part>) -> String {
    if xs.is_empty() {
        String::new()
    } else {
        xs[1..].iter().fold(format!("{}",xs.get(0).unwrap()), |s,x| format!("{}, {}",s,x))
    }
}

pub fn formatted_vec(xs: &Vec<usize>) -> String {
    formatted_vec_of_parts(&vec_to_parts(xs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_to_parts() {
        assert_eq!(vec![Part::Singleton(1)], vec_to_parts(&vec![1]));
        assert_eq!(vec![Part::Range(Range{start:1,end:5})], vec_to_parts(&vec![1,2,4,3,5]));
        assert_eq!(vec![Part::Singleton(1), Part::Range(Range{start:3,end:5})], vec_to_parts(&vec![1,3,4,5]));
        assert_eq!(vec![Part::Singleton(1), Part::Singleton(3)], vec_to_parts(&vec![3, 1]));
    }

    #[test]
    fn test_formatted_vec_of_parts() {
        assert_eq!(formatted_vec_of_parts(&vec![Part::Singleton(1)]), "1");
        assert_eq!(formatted_vec_of_parts(&vec![Part::Range(Range{start:1,end:5})]), "1-5");
        assert_eq!(formatted_vec_of_parts(&vec![Part::Singleton(1), Part::Range(Range{start:3,end:5})]), "1, 3-5");
        assert_eq!(formatted_vec_of_parts(&vec![Part::Singleton(1), Part::Singleton(3)]), "1, 3");
    }
}