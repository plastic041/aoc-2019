use winnow::{ModalResult, Parser, ascii::dec_uint, combinator::seq};

advent_of_code::solution!(12);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fabric {
    id: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Fabric {
    fn get_coords(self) -> impl Iterator<Item = usize> {
        (0..self.w * self.h).map(move |n| {
            (n.div_euclid(self.w) * 1000) + (n.rem_euclid(self.w)) + (self.y * 1000) + (self.x)
        })
    }
}

/// `#123 @ x,y: wxh`
fn parse_fabric(input: &mut &str) -> ModalResult<Fabric> {
    seq! {Fabric {
        _: '#',
        id: dec_uint,
        _: " @ ",

        x: dec_uint,
        _: ',',
        y: dec_uint,
        _: ": ",

        w: dec_uint,
        _: 'x',
        h: dec_uint,
    }}
    .parse_next(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let fabrics = input.lines().map(|line| parse_fabric.parse(line).unwrap());
    let mut counts: Vec<usize> = vec![0; 1000 * 1000];

    for fabric in fabrics {
        for coord in fabric.get_coords() {
            counts[coord] += 1;
        }
    }

    Some(counts.iter().filter(|count| **count > 1).count())
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fabric_parse() {
        let fabric = Fabric {
            id: 123,
            x: 3,
            y: 2,
            w: 5,
            h: 4,
        };
        let mut parsed = "#123 @ 3,2: 5x4";
        assert_eq!(fabric, parse_fabric(&mut parsed).unwrap());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
