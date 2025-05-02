// Wow I hate how I did this... Felt easier than any alternatives though.

pub fn main() {
    // 1, 2, 5, 10, 20, 50, 100, 200
    let mut count = 0;
    let mut remaining = 200;
    for twohundred in 0..=1 {
        remaining -= 200*twohundred;
        for hundred in 0..=remaining/100 {
            remaining -= 100*hundred;
            for fifty in 0..=remaining/50 {
                remaining -= 50*fifty;
                for twenty in 0..=remaining/20 {
                    remaining -= 20*twenty;
                    for ten in 0..=remaining/10 {
                        remaining -= 10*ten;
                        for five in 0..=remaining/5 {
                            remaining -= 5*five;
                            for _two in 0..=remaining/2 {
                                count += 1;
                            }
                            remaining += 5*five;
                        }
                        remaining += 10*ten;
                    }
                    remaining += 20*twenty;
                }
                remaining += 50*fifty;
            }
            remaining += 100*hundred;
        }
        remaining += 200*twohundred;
    }
    println!("{count}");
}