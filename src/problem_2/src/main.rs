fn main() {
    println!("{},{}",fib_even_total_bad(4000000),fib_even_total(4000000));
}

fn fib(n: i32) -> i32 {
    let mut counter = 1;
    let mut xm = 0;
    let mut xc = 1;
    while counter!=n {
        (xm,xc,counter) = (xc,xm+xc,counter+1);
    }
    xc
}

// bad way (probably will not finish running (does))
fn fib_even_total_bad(max: i32) -> i32 {
    let mut counter = 1;
    let mut curr = 0;
    let mut total = 0;
    while curr <= max {
        (curr,counter) = (fib(counter),counter+1);
        if curr % 2 == 0 {
            total += curr;
        }
    }
    total
}

fn fib_even_total(max:i32) -> i32 {
    let mut x0 = 0;
    let mut x1 = 1;
    let mut total = 0;
    while x1 <= max {
        if x1 % 2 == 0 {
            (x0,x1,total) = (x1,x0+x1,total+x1);
        } else {
            (x0,x1) = (x1,x0+x1);
        }
    }
    total
}

