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
    // let x = hold_time
    // x * (t - x) = dist
    // xt - x^2 - dist = 0
    // take derivative
    // t - 2x = 0
    // x = t / 2 (peak distance)
    let peak = tot_time / 2;
    // binary search left side of upside down parabola
    let min_value = {
        let mut start = 1;
        let mut end = peak;
        while start < end {
            let m = (end - start) / 2 + start;
            if m * (tot_time - m) <= distance {
                start = m + 1;
            } else {
                end = m;
            }
        }
        start
    };
    // binary search right side
    let max_value = {
        let mut start = peak;
        let mut end = tot_time - 1;
        while start < end {
            let m = (end - start + 1) / 2 + start;
            if m * (tot_time - m) <= distance {
                end = m - 1;
            } else {
                start = m;
            }
        }
        start
    };

    max_value - min_value + 1
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
