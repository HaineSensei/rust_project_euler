
fn tens_digits(n:u16, zero: bool) -> String {
    let n = n%100;
    if n < 20 {
        match n {
            0 => {
                match zero {
                    true => "zero".into(),
                    false => String::new()
                }
            },
            1 => "one".into(),
            2 => "two".into(),
            3 => "three".into(),
            4 => "four".into(),
            5 => "five".into(),
            6 => "six".into(),
            7 => "seven".into(),
            8 => "eight".into(),
            9 => "nine".into(),
            10 => "ten".into(),
            11 => "eleven".into(),
            12 => "twelve".into(),
            13 => "thirteen".into(),
            14 => "fourteen".into(),
            15 => "fifteen".into(),
            16 => "sixteen".into(),
            17 => "seventeen".into(),
            18 => "eighteen".into(),
            _ => "nineteen".into(),
        }
    } else {
        let [tens,units] = final_two(n.to_string().chars()).unwrap();
        let mut out;
        match tens {
            '2' => {
                out = "twenty".to_string();
            },
            '3' => {
                out = "thirty".to_string();
            },
            '4' => {
                out = "forty".to_string();
            },
            '5' => {
                out = "fifty".to_string();
            },
            '6' => {
                out = "sixty".to_string();
            },
            '7' => {
                out = "seventy".to_string();
            },
            '8' => {
                out = "eighty".to_string();
            },
            '9' => {
                out = "ninety".to_string();
            },
            _ => {
                out = "this isn't real!".to_string();
            }
        }
        match units {
            '0' => {},
            '1' => {
                out.push_str("-one");
            },
            '2' => {
                out.push_str("-two");
            },
            '3' => {
                out.push_str("-three");
            },
            '4' => {
                out.push_str("-four");
            },
            '5' => {
                out.push_str("-five");
            },
            '6' => {
                out.push_str("-six");
            },
            '7' => {
                out.push_str("-seven");
            },
            '8' => {
                out.push_str("-eight");
            },
            _ => {
                out.push_str("-nine");
            }
        }
        out
    }
}

fn in_words(n:u16) -> String {
    if n == 1000 {
        return "one thousand".into();
    }
    if n < 100 {
        tens_digits(n, true)
    } else {
        let tens_and_digits = tens_digits(n,false);
        let leading = n.to_string().chars().next().unwrap().to_digit(10).unwrap();
        let mut out = tens_digits(leading as u16, true);
        out.push_str(" hundred");
        match tens_and_digits.as_str() {
            "" => out,
            x => {
                out.push_str(" and ");
                out.push_str(x);
                out
            }
        }
    }
}

fn final_two<T: Default + Copy>(xs: impl IntoIterator<Item = T>) -> Option<[T; 2]> {
    let mut out = [T::default(), T::default()];
    let mut count: usize = 0;
    for x in xs {
        out = [out[1],x];
        count += 1;
    }
    if count < 2 {
        None
    } else {
        Some(out)
    }
}

fn significant(x: char) -> bool {
    match x {
        ' ' => false,
        '-' => false,
        _ => true
    }
}

fn count_significant_chars(text: &str) -> usize {
    text.to_string()
        .chars()
        .filter(|x: &char| significant(*x))
        .count()
}

pub fn main() {
    let total: usize = (1..=1000)
        .map(in_words)
        .map(|x: String| count_significant_chars(&x))
        .sum();
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_final_two() {
        assert_eq!(final_two(34.to_string().chars()),Some(['3','4']));
    }

    #[test]
    fn test_tens_digits() {
        assert_eq!(tens_digits(1,true),"one");
        assert_eq!(tens_digits(34,true),"thirty-four");
    }

    #[test]
    fn test_in_words_small_numbers() {
        assert_eq!(in_words(1), "one");
        assert_eq!(in_words(2), "two");
        assert_eq!(in_words(5), "five");
        assert_eq!(in_words(9), "nine");
    }
    
    #[test]
    fn test_in_words_teens() {
        assert_eq!(in_words(10), "ten");
        assert_eq!(in_words(11), "eleven");
        assert_eq!(in_words(15), "fifteen");
        assert_eq!(in_words(19), "nineteen");
    }
    
    #[test]
    fn test_in_words_tens() {
        assert_eq!(in_words(20), "twenty");
        assert_eq!(in_words(30), "thirty");
        assert_eq!(in_words(42), "forty-two");
        assert_eq!(in_words(99), "ninety-nine");
    }
    
    #[test]
    fn test_in_words_hundreds() {
        assert_eq!(in_words(100), "one hundred");
        assert_eq!(in_words(200), "two hundred");
        assert_eq!(in_words(900), "nine hundred");
    }
    
    #[test]
    fn test_in_words_hundreds_and_units() {
        assert_eq!(in_words(101), "one hundred and one");
        assert_eq!(in_words(202), "two hundred and two");
        assert_eq!(in_words(304), "three hundred and four");
    }
    
    #[test]
    fn test_in_words_hundreds_and_teens() {
        assert_eq!(in_words(111), "one hundred and eleven");
        assert_eq!(in_words(213), "two hundred and thirteen");
        assert_eq!(in_words(415), "four hundred and fifteen");
    }
    
    #[test]
    fn test_in_words_hundreds_and_tens() {
        assert_eq!(in_words(120), "one hundred and twenty");
        assert_eq!(in_words(230), "two hundred and thirty");
        assert_eq!(in_words(450), "four hundred and fifty");
    }
    
    #[test]
    fn test_in_words_hundreds_tens_and_units() {
        assert_eq!(in_words(123), "one hundred and twenty-three");
        assert_eq!(in_words(234), "two hundred and thirty-four");
        assert_eq!(in_words(456), "four hundred and fifty-six");
        assert_eq!(in_words(789), "seven hundred and eighty-nine");
    }
    
    #[test]
    fn test_in_words_thousand() {
        assert_eq!(in_words(1000), "one thousand");
    }
    
    #[test]
    fn test_problem_examples() {
        // Test specific examples from the problem statement
        assert_eq!(count_significant_chars(&in_words(342)), 23);
        assert_eq!(count_significant_chars(&in_words(115)), 20);
    }
    
    #[test]
    fn test_count_significant_chars() {
        // Test character counting functionality
        assert_eq!(count_significant_chars("one"), 3);
        assert_eq!(count_significant_chars("twenty-two"), 9); // Counts "twentytwo"
        assert_eq!(count_significant_chars("one hundred and fifteen"), 20);
    }
    
    #[test]
    fn test_sum_for_small_range() {
        // Test sum for numbers 1 to 5 as given in the example
        let sum: usize = (1..=5)
            .map(in_words)
            .map(|x: String| count_significant_chars(&x))
            .sum();
        assert_eq!(sum, 19); // "one" + "two" + "three" + "four" + "five" = 3+3+5+4+4 = 19
    }
    
    #[test]
    fn test_tens_digits_zero_handling() {
        // Test zero handling
        assert_eq!(tens_digits(0, true), "zero");
        assert_eq!(tens_digits(0, false), "");
        
        // Test when zero appears in tens place
        assert_eq!(tens_digits(101 % 100, false), "one");
    }
    
    #[test]
    fn test_debug_output() {
        // Output some specific numbers to manually verify
        println!("342 in words: {}", in_words(342));
        println!("342 letter count: {}", count_significant_chars(&in_words(342)));
        
        println!("115 in words: {}", in_words(115));
        println!("115 letter count: {}", count_significant_chars(&in_words(115)));
        
        println!("1000 in words: {}", in_words(1000));
        println!("1000 letter count: {}", count_significant_chars(&in_words(1000)));
    }
}