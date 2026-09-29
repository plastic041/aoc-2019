use itertools::Itertools;

advent_of_code::solution!(4);

fn check1(number: u32) -> bool {
    let mut valid = false;

    let mut digits = Vec::with_capacity(6);

    for n in 0..6 {
        let tens = 10u32.pow(5 - n);
        digits.push((number / tens) % 10);
    }

    let mut iter = digits.windows(2);
    while let Some([a, b]) = iter.next() {
        if a == b {
            valid = true;
        }

        if b < a {
            return false;
        }
    }

    return valid;
}

pub fn part_one(input: &str) -> Option<u64> {
    let (from, to) = input
        .split("-")
        .map(|n| n.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut count = 0;

    for number in from..=to {
        if check1(number) {
            count += 1;
        }
    }

    Some(count)
}

fn check2(number: u32) -> bool {
    let mut digits = Vec::with_capacity(8);

    digits.push(None);

    for n in 0..6 {
        let tens = 10u32.pow(5 - n);
        digits.push(Some((number / tens) % 10));
    }

    digits.push(None);

    let mut has_double = false;

    let mut iter = digits.windows(4);
    while let Some([n1, n2, n3, n4]) = iter.next() {
        if n3 < n2 {
            return false;
        }

        if n1 != n2 && n2 == n3 && n3 != n4 {
            has_double = true
        }
    }

    return has_double;
}

pub fn part_two(input: &str) -> Option<u64> {
    let (from, to) = input
        .split("-")
        .map(|n| n.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut count = 0;

    for number in from..=to {
        if check2(number) {
            count += 1;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(check1(123789), false);
        assert_eq!(check1(223450), false);
        assert_eq!(check1(111111), true);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(check2(112233), true);
        assert_eq!(check2(123444), false);
        assert_eq!(check2(111122), true);
    }
}
