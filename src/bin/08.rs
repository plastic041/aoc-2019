use itertools::Itertools;

advent_of_code::solution!(8);

fn split_by_size(input: &str, size: usize) -> Vec<String> {
    let mut lines = Vec::with_capacity(input.len() / size);

    for chunk in &input.chars().chunks(size) {
        lines.push(String::from_iter(chunk));
    }

    lines
}

fn count_char(input: &str, c: &char) -> usize {
    input.chars().filter(|ch| ch == c).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let layers = split_by_size(input, 25 * 6);
    let mut sorted_layers = layers.iter().sorted_by(|a, b| {
        let azeros = count_char(a, &'0');
        let bzeros = count_char(b, &'0');
        azeros.cmp(&bzeros)
    });
    let first = sorted_layers.next().unwrap();

    let firstones = count_char(first, &'1');
    let firsttwos = count_char(first, &'2');

    Some(firstones * firsttwos)
}

fn find_color(layers: Vec<Vec<char>>, index: usize) -> char {
    let mut iter = layers
        .iter()
        .map(|layer| layer[index])
        .skip_while(|ch| *ch == '2');
    iter.next().unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let layers = split_by_size(input, 25 * 6);
    let mut colors = Vec::with_capacity(25 * 6);
    for i in 0..25 * 6 {
        let color = find_color(
            layers.iter().map(|layer| layer.chars().collect()).collect(),
            i,
        );
        colors.push(color);
    }

    let image = split_by_size(&colors.into_iter().join(""), 25);

    println!("{}", image.join("\n"));

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = split_by_size(&advent_of_code::template::read_file("examples", DAY), 3);
        assert_eq!(result, vec!["123", "456", "789", "012"]);
    }
}
