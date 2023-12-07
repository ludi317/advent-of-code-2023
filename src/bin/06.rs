advent_of_code::solution!(6);
use ::advent_of_code::{get_nums, get_nums_as_1};

pub fn part_one(input: &str) -> Option<isize> {
    let mut lines = input.lines();
    let tot_time = get_nums(lines.next().unwrap());
    let distance = get_nums(lines.next().unwrap());
    let num_races = tot_time.len();
    let mut prod = 1;
    for i in 0..num_races {
        prod *= num_wins(tot_time[i], distance[i]);
    }

    Some(prod)
}

fn num_wins(tot_time: isize, distance: isize) -> isize {
    // quadratic formula
    let part = ((tot_time * tot_time - 4 * (-1) * (-distance)) as f64).sqrt();
    let higher = ((-tot_time as f64 - part) / -2f64 - 1.0).ceil() as isize;
    let lower = ((-tot_time as f64 + part) / -2f64 + 1.0).floor() as isize;
    return higher - lower + 1;
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut lines = input.lines();
    let tot_time = get_nums_as_1(lines.next().unwrap());
    let distance = get_nums_as_1(lines.next().unwrap());
    Some(num_wins(tot_time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
