use core::range;
use itertools::Itertools;
use std::num::ParseIntError;

advent_of_code::solution!(4);

struct Password {
    numbers: Vec<u8>,
}

impl Password {
    fn new(numbers: String) -> Self {
        Self {
            numbers: numbers
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect_vec(),
        }
    }

    fn get_number(&self) -> Result<u32, ParseIntError> {
        self.numbers.iter().join("").parse::<u32>()
    }

    fn is_valid_length(&self) -> bool {
        self.numbers.len() == 6
    }

    fn is_valid_range(&self, from: u32, to: u32) -> bool {
        if let Ok(number) = self.get_number() {
            from <= number && number <= to
        } else {
            false
        }
    }

    fn is_valid_adjacent(&self) -> bool {
        for nums in self.numbers.windows(2) {
            let a = &nums[0];
            let b = &nums[1];

            if a == b {
                return true;
            }
        }

        false
    }

    fn is_valid_increase(&self) -> bool {
        for nums in self.numbers.windows(2) {
            let a = &nums[0];
            let b = &nums[1];

            if a > b {
                return false;
            }
        }

        true
    }

    fn is_valid(&self, from: u32, to: u32) -> bool {
        self.is_valid_length()
            && self.is_valid_range(from, to)
            && self.is_valid_adjacent()
            && self.is_valid_increase()
    }
}

pub fn part_one(_input: &str) -> Option<u64> {
    // 231832-767346
    let from = 231832;
    let to = 767346;

    let mut count = 0;

    for number in from..=to {
        let pw = Password::new(number.to_string());
        if pw.is_valid(from, to) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);

        assert!(Password::new("111111".to_string()).is_valid(0, 999999));
        assert!(!Password::new("223450".to_string()).is_valid(0, 999999));
        assert!(!Password::new("123789".to_string()).is_valid(0, 999999));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
