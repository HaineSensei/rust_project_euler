#[derive(Clone, Copy, PartialEq, Debug)]
struct Month {
    month: u8,
    year: u16
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Date {
    day: u8,
    month: Month
}

impl Month {
    // Jan 1 31, Feb 2 28, Mar 3 31, Apr 4 30, May 5 31, Jun 6 30, Jul 7 31, Aug 8 31, Sep 9 30, Oct 10 31, Nov 11 30, Dec 12 31
    fn month_length(self) -> u8 {
        if [1,3,5,7,8,10,12].contains(&self.month) {
            31
        } else if self.month != 2 {
            30
        } else {
            if self.year%4 != 0 || (self.year%100 == 0 && self.year%400 != 0) {
                28
            } else {
                29
            }
        }
    }

    fn successor(self) -> Self {
        if self.month != 12 {
            return Self {
                month: self.month+1,
                year: self.year
            }
        } else {
            return Self {
                month: 1,
                year: self.year + 1
            }
        }
    }
}

impl Date {
    // only handles small errors of + 1 month
    fn normalised(self) -> Self {
        let mut length = self.month.month_length();
        let mut curr = self;
        while !(curr.day <= length) {
            curr = Self {
                day: curr.day - length,
                month: curr.month.successor()
            };
            length = curr.month.month_length();
        }
        curr
    }

    fn add(self, n: u8) -> Date {
        Self {
            day: self.day + n,
            month: self.month
        }.normalised()
    }
}

pub fn main() {
    let mut curr: Date = Date {
        day: 31,
        month: Month {
            month: 12,
            year: 1899
        }
    };
    let mut count: usize = 0;
    while curr.month.year < 2001 {
        if curr.day == 1 && curr.month.year > 1900 {
            count += 1;
        }
        curr = curr.add(7);
    }
    println!("{count}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_lengths_regular_year() {
        // Test a non-leap year (1901)
        assert_eq!(Month { month: 1, year: 1901 }.month_length(), 31); // January
        assert_eq!(Month { month: 2, year: 1901 }.month_length(), 28); // February (non-leap)
        assert_eq!(Month { month: 3, year: 1901 }.month_length(), 31); // March
        assert_eq!(Month { month: 4, year: 1901 }.month_length(), 30); // April
        assert_eq!(Month { month: 5, year: 1901 }.month_length(), 31); // May
        assert_eq!(Month { month: 6, year: 1901 }.month_length(), 30); // June
        assert_eq!(Month { month: 7, year: 1901 }.month_length(), 31); // July
        assert_eq!(Month { month: 8, year: 1901 }.month_length(), 31); // August
        assert_eq!(Month { month: 9, year: 1901 }.month_length(), 30); // September
        assert_eq!(Month { month: 10, year: 1901 }.month_length(), 31); // October
        assert_eq!(Month { month: 11, year: 1901 }.month_length(), 30); // November
        assert_eq!(Month { month: 12, year: 1901 }.month_length(), 31); // December
    }

    #[test]
    fn test_february_leap_years() {
        // Regular leap year (divisible by 4)
        assert_eq!(Month { month: 2, year: 1904 }.month_length(), 29);
        assert_eq!(Month { month: 2, year: 2000 }.month_length(), 29); // Also divisible by 400
        
        // Non-leap year (not divisible by 4)
        assert_eq!(Month { month: 2, year: 1901 }.month_length(), 28);
        
        // Century not divisible by 400 (not a leap year)
        assert_eq!(Month { month: 2, year: 1900 }.month_length(), 28);
        assert_eq!(Month { month: 2, year: 1800 }.month_length(), 28);
    }

    #[test]
    fn test_edge_cases() {
        // Test the year 2000 (leap year, divisible by 400)
        assert_eq!(Month { month: 2, year: 2000 }.month_length(), 29);
        
        // Test the year 1900 (not a leap year, divisible by 100 but not by 400)
        assert_eq!(Month { month: 2, year: 1900 }.month_length(), 28);
        
        // Test the year 2100 (not a leap year, divisible by 100 but not by 400)
        assert_eq!(Month { month: 2, year: 2100 }.month_length(), 28);
    }

    #[test]
    fn test_month_successor() {
        // Regular month succession within the same year
        assert_eq!(
            Month { month: 1, year: 1901 }.successor(),
            Month { month: 2, year: 1901 }
        );
        assert_eq!(
            Month { month: 6, year: 1901 }.successor(),
            Month { month: 7, year: 1901 }
        );
        assert_eq!(
            Month { month: 11, year: 1901 }.successor(),
            Month { month: 12, year: 1901 }
        );
        
        // Year transition
        assert_eq!(
            Month { month: 12, year: 1901 }.successor(),
            Month { month: 1, year: 1902 }
        );
        assert_eq!(
            Month { month: 12, year: 1999 }.successor(),
            Month { month: 1, year: 2000 }
        );
        
        // Century transition
        assert_eq!(
            Month { month: 12, year: 1899 }.successor(),
            Month { month: 1, year: 1900 }
        );
        assert_eq!(
            Month { month: 12, year: 1999 }.successor(),
            Month { month: 1, year: 2000 }
        );
    }

    #[test]
    fn test_normalise_date() {
        assert_eq!(Date {
            day: 33,
            month: Month {
                month: 1,
                year: 1983
            }
        }.normalised(), Date {
            day: 2,
            month: Month {
                month: 2,
                year: 1983
            }
        });
        assert_eq!(Date {
            day: 31,
            month: Month {
                month: 1,
                year: 1983
            }
        }.normalised(), Date {
            day: 31,
            month: Month {
                month: 1,
                year: 1983
            }
        });
        assert_eq!(Date {
            day: 29,
            month: Month {
                month: 2,
                year: 1983
            }
        }.normalised(), Date {
            day: 1,
            month: Month {
                month: 3,
                year: 1983
            }
        });
    }
}