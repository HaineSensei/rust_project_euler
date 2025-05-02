use std::{collections::HashSet, iter::from_fn, ops::{Add, Sub}};


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coins {
    ones:u8,
    twos:u8,
    fives:u8,
    tens:u8,
    twenties:u8,
    fifties:u8,
    pounds:u8,
    twopounds:u8
}

impl Add for Coins {
    type Output = Coins;

    fn add(self, rhs: Self) -> Self::Output {
        Coins {
            ones: self.ones + rhs.ones,
            twos: self.twos + rhs.twos,
            fives: self.fives + rhs.fives,
            tens: self.tens + rhs.tens,
            twenties: self.twenties + rhs.twenties,
            fifties: self.fifties + rhs.fifties,
            pounds: self.pounds + rhs.pounds,
            twopounds: self.twopounds + rhs.twopounds,
            
        }
    }
}

const ONE: Coins = Coins{ones:1, twos: 0, fives: 0, tens: 0, twenties: 0, fifties: 0, pounds: 0, twopounds: 0 };
const TWO: Coins = Coins{ones:0, twos: 1, fives: 0, tens: 0, twenties: 0, fifties: 0, pounds: 0, twopounds: 0 };
const FIVE: Coins = Coins{ones:0, twos: 0, fives: 1, tens: 0, twenties: 0, fifties: 0, pounds: 0, twopounds: 0 };
const TEN: Coins = Coins{ones:0, twos: 0, fives: 0, tens: 1, twenties: 0, fifties: 0, pounds: 0, twopounds: 0 };
const TWENTY: Coins = Coins{ones:0, twos: 0, fives: 0, tens: 0, twenties: 1, fifties: 0, pounds: 0, twopounds: 0 };
const FIFTY: Coins = Coins{ones:0, twos: 0, fives: 0, tens: 0, twenties: 0, fifties: 1, pounds: 0, twopounds: 0 };
const POUND: Coins = Coins{ones:0, twos: 0, fives: 0, tens: 0, twenties: 0, fifties: 0, pounds: 1, twopounds: 0 };
const TWOPOUND: Coins = Coins{ones:0, twos: 0, fives: 0, tens: 0, twenties: 0, fifties: 0, pounds: 0, twopounds: 1 };

#[allow(dead_code)]
const COINS: [Coins; 8] = [ONE, TWO, FIVE, TEN, TWENTY, FIFTY, POUND, TWOPOUND];
const COINS_REV: [Coins; 8] = [TWOPOUND, POUND, FIFTY, TWENTY, TEN, FIVE, TWO, ONE];

impl Sub for Coins {
    type Output = Coins;

    fn sub(self, rhs: Self) -> Self::Output {
        Coins {
            ones: self.ones - rhs.ones,
            twos: self.twos - rhs.twos,
            fives: self.fives - rhs.fives,
            tens: self.tens - rhs.tens,
            twenties: self.twenties - rhs.twenties,
            fifties: self.fifties - rhs.fifties,
            pounds: self.pounds - rhs.pounds,
            twopounds: self.twopounds - rhs.twopounds,
            
        }
    }
}

fn coin_splits<'a>(n: u8, coins_available: &mut Vec<Coins>) -> HashSet<Coins> {
    let mut remaining = coins_available.clone();
    let mut curr = remaining.pop();
    let mut out = HashSet::new();
    while !remaining.is_empty() {
        match curr {
            Some(coin) => {
                match coin.value().cmp(&n) {
                    std::cmp::Ordering::Less => {
                        for coins in coin_splits(n, coins_available) {
                            out.insert(coins+ coin);
                        }
                        curr = None;
                    },
                    std::cmp::Ordering::Equal => {
                        out.insert(coin);
                        curr = None;
                    },
                    std::cmp::Ordering::Greater => break,
                }
            },
            None => {
                curr = remaining.pop();
            },
        }

    }
    out
}

impl Default for Coins {
    fn default() -> Self {
        Self { ones: Default::default(), twos: Default::default(), fives: Default::default(), tens: Default::default(), twenties: Default::default(), fifties: Default::default(), pounds: Default::default(), twopounds: Default::default() }
    }
}

impl Coins {
    fn value(self) -> u8 {
        self.ones + self.twos*2 + self.fives*5 + self.tens*10 + self.twenties*20 + self.fifties*50 + self.pounds*100 + self.twopounds*200
    }
}

pub fn main() {
    println!("{}",coin_splits(100, &mut COINS_REV.to_vec()).len());
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coin_splits() {
        for coins in coin_splits(5,&mut COINS_REV.to_vec()) {
            println!("{coins:?}");
        }
    }
}
