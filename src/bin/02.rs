advent_of_code::solution!(2);

use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::digit1, multi::separated_list1,
};

fn parse_nums(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, nums) = separated_list1(tag(","), digit1.map_res(str::parse)).parse(input)?;

    Ok((input, nums))
}

fn step(index: usize, numbers: &mut [u32]) -> Option<usize> {
    let number = numbers[index];

    match number {
        1 => {
            let input1 = numbers[numbers[index + 1] as usize];
            let input2 = numbers[numbers[index + 2] as usize];
            numbers[numbers[index + 3] as usize] = input1 + input2;
            return Some(index + 4);
        }
        2 => {
            let input1 = numbers[numbers[index + 1] as usize];
            let input2 = numbers[numbers[index + 2] as usize];
            numbers[numbers[index + 3] as usize] = input1 * input2;
            return Some(index + 4);
        }
        99 => return None,
        _ => panic!("Unknown opcode: {} at {}", number, index),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut numbers) = parse_nums(input).unwrap();
    numbers[1] = 12;
    numbers[2] = 2;

    let mut cursor = 0;
    while let Some(next) = step(cursor, &mut numbers) {
        cursor = next;
    }

    Some(numbers[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, numbers) = parse_nums(input).unwrap();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut nums = numbers.clone();

            nums[1] = noun;
            nums[2] = verb;

            let mut cursor = 0;
            while let Some(next) = step(cursor, &mut nums) {
                cursor = next;
            }

            if nums[0] == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
