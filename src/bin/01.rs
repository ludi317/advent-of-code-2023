advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_calib_vals(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_calib_vals2(input))
}

fn sum_calib_vals(s: &str) -> u32 {
    s.lines()
        .map(|line| {
            let bs = line.as_bytes();

            let first_digit = (bs.iter().find(|&b| b.is_ascii_digit()).unwrap() - b'0') as u32;

            let last_digit = (bs.iter().rev().find(|&b| b.is_ascii_digit()).unwrap() - b'0') as u32;

            first_digit * 10 + last_digit
        })
        .sum()
}

fn sum_calib_vals2(s: &str) -> u32 {
    let mut words: HashMap<&str, u32> = HashMap::new();
    words.insert("one", 1);
    words.insert("two", 2);
    words.insert("three", 3);
    words.insert("four", 4);
    words.insert("five", 5);
    words.insert("six", 6);
    words.insert("seven", 7);
    words.insert("eight", 8);
    words.insert("nine", 9);

    s.lines()
        .map(|line| {
            let bs = line.as_bytes();

            // Find the first digit and its index
            let digit_find = bs.iter().enumerate().find(|&(_, &b)| b.is_ascii_digit());

            // Find the first word and its index
            let word_find = words
                .iter()
                .filter_map(|(word, &val)| line.find(word).map(|i| (i, val)))
                .min_by_key(|&(i, _)| i);

            // Choose the value with the lower index
            let first_num = match (digit_find, word_find) {
                (Some((i, &b)), Some((wi, wv))) => {
                    if i < wi {
                        (b - b'0') as u32
                    } else {
                        wv
                    }
                }
                (Some((_, &b)), None) => (b - b'0') as u32,
                (None, Some((_, wv))) => wv,
                (None, None) => panic!("did not find word or digit in line"),
            };

            // Find the last digit and its reverse-index
            let digit_find = bs
                .iter()
                .rev()
                .enumerate()
                .find(|&(_, &b)| b.is_ascii_digit());

            // Find the last word and its index
            let word_find = words
                .iter()
                .filter_map(|(word, &val)| line.rfind(word).map(|i| (i, val)))
                .max_by_key(|&(i, _)| i);

            let last_num = match (digit_find, word_find) {
                (Some((i, &b)), Some((wi, wv))) => {
                    if bs.len() - i - 1 > wi {
                        (b - b'0') as u32
                    } else {
                        wv
                    }
                }
                (Some((_, &b)), None) => (b - b'0') as u32,
                (None, Some((_, wv))) => wv,
                (None, None) => panic!("did not find word or digit in line"),
            };

            first_num * 10 + last_num
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
