use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, one_of},
    combinator::{map, map_parser, map_res},
    multi::separated_list1,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

advent_of_code::solution!(3);

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Path {
    direction: Direction,
    distance: i32,
}

/// Parses `"{R | L | U | D}{digit}{digit}"` into `Path`
fn parse_path(input: &str) -> IResult<&str, Path> {
    let (input, direction) = map(one_of("RLUD"), |dir| match dir {
        'R' => Direction::Right,
        'L' => Direction::Left,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => panic!("Unrecognised char"),
    })
    .parse(input)?;
    let (input, distance) = map_res(digit1, |digit: &str| digit.parse::<i32>()).parse(input)?;

    Ok((
        input,
        Path {
            direction,
            distance,
        },
    ))
}

fn parse_paths(input: &str) -> IResult<&str, Vec<Path>> {
    let (input, paths) =
        separated_list1(tag(","), map_parser(alphanumeric1, parse_path)).parse(input)?;

    Ok((input, paths))
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn trail(paths: &[Path]) -> HashSet<Coordinate> {
    let mut coordinates = HashSet::new();
    let mut current_pos = Coordinate { x: 0, y: 0 };

    for path in paths {
        for _ in 0..path.distance {
            match path.direction {
                Direction::Right => {
                    current_pos.x += 1;
                    coordinates.insert(current_pos.clone());
                }
                Direction::Left => {
                    current_pos.x -= 1;
                    coordinates.insert(current_pos.clone());
                }
                Direction::Up => {
                    current_pos.y += 1;
                    coordinates.insert(current_pos.clone());
                }
                Direction::Down => {
                    current_pos.y -= 1;
                    coordinates.insert(current_pos.clone());
                }
            }
        }
    }

    coordinates
}

fn trail2(paths: &[Path]) -> HashMap<Coordinate, u32> {
    let mut coordinates = HashMap::new();
    let mut current_pos = Coordinate { x: 0, y: 0 };
    let mut traveled_distance = 0;

    for path in paths {
        for _ in 0..path.distance {
            traveled_distance += 1;

            match path.direction {
                Direction::Right => {
                    current_pos.x += 1;
                }
                Direction::Left => {
                    current_pos.x -= 1;
                }
                Direction::Up => {
                    current_pos.y += 1;
                }
                Direction::Down => {
                    current_pos.y -= 1;
                }
            }

            coordinates.insert(current_pos.clone(), traveled_distance);
        }
    }

    coordinates
}

pub fn part_one(input: &str) -> Option<i32> {
    let (wire1, wire2) = input.lines().collect_tuple().unwrap();

    let (_, paths1) = parse_paths(wire1).unwrap();
    let (_, paths2) = parse_paths(wire2).unwrap();

    let coordinates1 = trail(&paths1);
    let coordinates2 = trail(&paths2);

    let intersections = coordinates1.intersection(&coordinates2).collect_vec();

    let min_distance = intersections
        .iter()
        .map(|&coordinate| coordinate.x.abs() + coordinate.y.abs())
        .min()
        .expect(&format!("{:?}", intersections));

    Some(min_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (wire1, wire2) = input.lines().collect_tuple().unwrap();

    let (_, paths1) = parse_paths(wire1).unwrap();
    let (_, paths2) = parse_paths(wire2).unwrap();

    let coordinates1 = trail2(&paths1);
    let coordinates2 = trail2(&paths2);

    let mut distances = vec![];

    for (coordinate, distance1) in coordinates1 {
        if let Some(distance2) = coordinates2.get(&coordinate) {
            distances.push(distance1 + distance2);
        }
    }

    let min_distance = distances.iter().min().unwrap();

    Some(*min_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
