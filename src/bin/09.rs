#![allow(dead_code)]

advent_of_code::solution!(9);

use winnow::{
    ModalResult, Parser,
    ascii::{dec_uint, space0},
    combinator::{delimited, eof, opt, peek, repeat_till, seq},
    token::{any, take_till},
};

#[derive(Debug, PartialEq)]
struct Message<'a> {
    name: &'a str,
    sentence: &'a str,
    number: Option<u32>,
}

fn number_in_parens(input: &mut &str) -> ModalResult<u32> {
    delimited('(', dec_uint, ')').parse_next(input)
}

fn sentence<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    repeat_till::<_, _, (), _, _, _, _>(0.., any, peek((opt(number_in_parens), space0, eof)))
        .take()
        .map(str::trim_end)
        .parse_next(input)
}

fn simple_parser<'a>(input: &mut &'a str) -> ModalResult<Message<'a>> {
    seq! {
        Message {
            name: take_till(0.., ':'),
            _: (':', space0),
            sentence: take_till(0.., '('),
            number: opt(number_in_parens)
        }
    }
    .parse_next(input)
}

pub fn part_one(_input: &str) -> Option<u64> {
    None
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut input = "Something: hello world(2)";
        let output = simple_parser.parse_next(&mut input);

        assert_eq!(
            output,
            Ok(Message {
                name: "Something",
                sentence: "hello world",
                number: Some(2)
            })
        );

        let mut input = "Something: hello world";
        let output = simple_parser.parse_next(&mut input);

        assert_eq!(
            output,
            Ok(Message {
                name: "Something",
                sentence: "hello world",
                number: None
            })
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
