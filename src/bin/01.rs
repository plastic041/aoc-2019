advent_of_code::solution!(1);

fn find_fuel(mass: u32) -> Option<u32> {
    mass.checked_div(3).and_then(|div| div.checked_sub(2))
}

fn find_fuel_rec(mass: u32) -> u32 {
    let mut fuel_sum = 0;
    let mut fuel = find_fuel(mass);

    loop {
        if let Some(f) = fuel {
            fuel_sum += f;
            fuel = find_fuel(f)
        } else {
            break;
        }
    }

    fuel_sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let fuel_sum: u32 = input
        .lines()
        .map(|line| find_fuel(line.parse::<u32>().unwrap()).unwrap())
        .sum();

    Some(fuel_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let fuel_sum: u32 = input
        .lines()
        .map(|line| find_fuel_rec(line.parse::<u32>().unwrap()))
        .sum();

    Some(fuel_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_fuel() {
        assert_eq!(find_fuel(100756), Some(33583));
    }

    #[test]
    fn test_find_fuel_rec() {
        assert_eq!(find_fuel_rec(1969), 966);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34241));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51316));
    }
}
