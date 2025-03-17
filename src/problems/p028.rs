const DIMENSION : usize = 1001;
const MIDPOINT : usize = 1001/2 + 1;

#[derive(Clone, Copy)]
enum Side {
    Right,
    Bottom,
    Left,
    Top,
}

fn starting(ring: usize) -> usize {
    let inner_square_dim = if ring == 0 {
        0
    } else {
        ring*2 - 1
    };
    inner_square_dim*inner_square_dim + 1
}

// At this point, I notice that I've been using 1 indexing for both x and y coordinates from the top left, down and right respectively.
fn side(x:usize, y:usize) -> Option<Side> {
    match (x.cmp(&y),x.cmp(&(DIMENSION-y + 1))) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(Side::Left),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Some(Side::Bottom),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(Side::Bottom),
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Some(Side::Left),
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => None, // x = y = MIDPOINT
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Some(Side::Right),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(Side::Top),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Some(Side::Top),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(Side::Right),
    }
}

fn edge_starting_point(side: Side, ring: usize) -> (usize, usize) {
    match side {
        Side::Right => (MIDPOINT + ring, MIDPOINT - ring + 1),
        Side::Bottom => (MIDPOINT + ring - 1, MIDPOINT + ring),
        Side::Left => (MIDPOINT - ring, MIDPOINT + ring - 1),
        Side::Top => (MIDPOINT - ring + 1, MIDPOINT - ring),
    }
}

fn distance_1d(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn distance_2d(p1: (usize,usize), p2: (usize,usize)) -> usize {
    std::cmp::max(distance_1d(p1.0, p2.0), distance_1d(p1.1, p2.1))
}

fn distance_along_edge(side: Side, ring: usize, x: usize, y: usize) -> usize {
    if x == y && x == MIDPOINT {
        0
    } else {
        let starting_point = edge_starting_point(side, ring);
        let p = (x,y);
        distance_2d(starting_point, p)
    }
}

fn side_length(ring: usize) -> usize {
    // looks like ring * 2 + 1, but we only count 1 vertex per side.
    ring*2 
}

fn value(x:usize, y:usize) -> usize {
    let (dx, dy);
    match (x < MIDPOINT, y < MIDPOINT) {
        (true, true) => {
            (dx, dy) = (MIDPOINT - x, MIDPOINT - y);
        },
        (true, false) => {
            (dx, dy) = (MIDPOINT - x, y - MIDPOINT);
        },
        (false, true) => {
            (dx, dy) = (x - MIDPOINT, MIDPOINT - y);
        },
        (false, false) => {
            (dx, dy) = (x - MIDPOINT, y - MIDPOINT);
        },
    }
    let ring: usize = std::cmp::max(dx,dy);
    match side(x,y) {
        Some(side) => {
            let mult;
            match side {
                Side::Right => {mult = 0;},
                Side::Bottom => {mult = 1;},
                Side::Left => {mult = 2;},
                Side::Top => {mult = 3;},
            }
            starting(ring) + side_length(ring)*mult + distance_along_edge(side, ring, x, y)
        },
        None => 1
    }
}

pub fn main() {
    let diagonal_points = (1..MIDPOINT)
        .map(|x| (x,x))
        .chain((1..MIDPOINT).map(|x| (x,DIMENSION + 1 - x)))
        .chain((MIDPOINT+1..=DIMENSION).map(|x| (x,x)))
        .chain((MIDPOINT..=DIMENSION).map(|x| (x, DIMENSION + 1 - x)));
    println!("{}", diagonal_points.map(|(x,y)|value(x,y)).sum::<usize>())
}
