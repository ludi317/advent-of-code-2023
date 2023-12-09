advent_of_code::solution!(2);

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::sequence::delimited;
use nom::IResult;

pub fn part_one(input: &str) -> Option<u32> {
    let (max_red, max_green, max_blue) = (12, 13, 14);
    let id_sum = input
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let cube = cube(line).unwrap().1;
            cube.min_blue <= max_blue && cube.min_red <= max_red && cube.min_green <= max_green
        })
        .map(|(i, _)| (i + 1) as u32)
        .sum();

    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let cube = cube(line).unwrap().1;
            cube.min_blue * cube.min_green * cube.min_red
        })
        .sum();
    Some(sum)
}

#[derive(Debug, PartialEq)]
struct Cube {
    min_red: u32,
    min_green: u32,
    min_blue: u32,
}

fn cube(mut input: &str) -> IResult<&str, Cube> {
    let mut a;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    // strip the "Game N:" prefix
    let colon_idx = input.find(":").unwrap();
    input = &input[colon_idx + 1..];
    while input != "" {
        (input, a) = delimited(alt((tag(", "), tag("; "), tag(" "))), digit1, tag(" "))(input)?;
        let num = a.parse().unwrap();
        (input, a) = alpha1(input)?;
        match a {
            "red" => red = red.max(num),
            "green" => green = green.max(num),
            "blue" => blue = blue.max(num),
            _ => unreachable!(),
        }
    }

    Ok((
        input,
        Cube {
            min_red: red,
            min_green: green,
            min_blue: blue,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
