use std::{collections::HashSet, fmt::Display, hash::Hash};

struct Frac {
    num: u16,
    den: u16,
}

impl Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}/{}",self.num,self.den)
    }
}

fn gcd(a:u16, b:u16) -> u16 {
    if a == 0 {
        b
    } else {
        gcd(b%a, a)
    }
}

impl Frac {
    fn new(n:u16,d:u16) -> Self {
        Self { num: n, den: d }
    }

    fn simplified(&self) -> Self {
        let fact = gcd(self.num, self.den);
        Self {
            num: self.num/fact,
            den: self.den/fact
        }
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        self.num*other.den == self.den*other.num
    }
}

impl Eq for Frac {}

impl Hash for Frac {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let simplified = self.simplified();
        (simplified.num,simplified.den).hash(state)
    }
}

pub fn main() {
    // AB/CA or BA/AC 
    // Key things: none of A,B,C can be 0.
    let mut digit_cancellings = HashSet::new();
    for a in 1..10 {
        for b in 1..10 {
            for c in 1..10 {
                // AB/CA
                let abca = Frac::new(a*10+b, c*10+a);
                let baac = Frac::new(b*10+a, a*10+c);
                let bc = Frac::new(b,c);
                if &abca == &bc {
                    digit_cancellings.insert((bc,abca));
                } else if &baac == &bc {
                    digit_cancellings.insert((bc,baac));
                }
            }
        }
    }

    println!("values:");
    for x in digit_cancellings.iter().filter(|(x,_)| x.num < x.den) {
        println!("{} {}", x.0, x.1);
    }
    println!("\nAnswer:");
    let simplified_dens_prod: u16 = digit_cancellings
        .iter()
        .map(|x|&x.0)
        .filter(|x| x.num < x.den)
        .fold(Frac::new(1,1), |x, y| Frac::new(x.num*y.num,x.den*y.den).simplified())
        .simplified()
        .den;
    println!("{simplified_dens_prod}");
}
