use std::ops::Range;

///////////////////////////////////////////////////////////////////////////////

struct DigitIterator {
    n: u64,
    count: u8
}

impl DigitIterator {
    pub fn new(n: u64) -> Self {
        let count = (n as f64).log10() as u8 + 1;
        DigitIterator { n, count }
    }
}

impl Iterator for DigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.count == 0 {
            return None;
        }

        let factor = 10_u64.pow(self.count as u32 - 1);
        let digit = (self.n / factor) % 10;

        self.count -= 1;

        Some(digit as u8)
    }
}

fn digits(n: u64) -> DigitIterator {
    DigitIterator::new(n)
}

///////////////////////////////////////////////////////////////////////////////

fn is_valid_part_one(n: u64) -> bool {
    let mut d = digits(n);
    let mut prev = d.next().expect("number out of range");
    let mut has_double = false;    

    for digit in d {
        if digit == prev { has_double = true }
        else if digit > prev { prev = digit }
        else if digit < prev { return false }
    }

    has_double
}

fn n_valid_part_one(range: Range<u64>) -> u64 {
    range.fold(0, |acc, n| acc + is_valid_part_one(n) as u64)
}

///////////////////////////////////////////////////////////////////////////////

fn is_valid_part_two(n: u64) -> bool {
    let mut d = digits(n);
    let mut prev = d.next().expect("number out of range");
    let mut has_double = false;
    let mut adjacent = 1;

    for digit in d {
        if digit == prev {
            adjacent += 1;
        } else {
            if adjacent == 2 {
                has_double = true;
            }
            adjacent = 1;

            if digit < prev { return false }
            
            prev = digit;
        }
    }

    has_double || adjacent == 2
}

fn n_valid_part_two(range: Range<u64>) -> u64 {
    range.fold(0, |acc, n| acc + is_valid_part_two(n) as u64)
}

///////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("part one: {}", n_valid_part_one(136760..595730));
    println!("part two: {}", n_valid_part_two(136760..595730));
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests_day4 {
    use super::*;

    #[test]
    fn digits() {
        let digits: Vec<u8> = DigitIterator::new(12345601).collect();
        assert_eq!(digits, vec![1,2,3,4,5,6,0,1]);
    }
}
