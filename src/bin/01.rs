advent_of_code::solution!(1);

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

    let words: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    s.lines()
        .map(|line| {
            let bs = line.as_bytes();

            // Find the first digit and its index
            let digit_find = bs.iter().enumerate().find(|&(_, &b)| b.is_ascii_digit());

            // Find the first word-digit and its index
            let word_find = words
                .iter()
                .enumerate()
                .filter_map(|(i, word)| line.find(word).map(|s_idx| (s_idx, i+1)))
                .min_by_key(|&(s_idx, _)| s_idx);

            // Choose the value with the lower index
            let first_num: usize = match (digit_find, word_find) {
                (Some((i, &b)), Some((wi, wv))) => {
                    if i < wi {
                        (b - b'0') as usize
                    } else {
                        wv
                    }
                }
                (Some((_, &b)), None) => (b - b'0') as usize,
                (None, Some((_, wv))) => wv,
                (None, None) => panic!("did not find word or digit in line"),
            };

            // Find the last digit and its reverse-index
            let digit_find = bs
                .iter()
                .rev()
                .enumerate()
                .find(|(_, b)| b.is_ascii_digit());

            // Find the last word and its index
            let word_find = words
                .iter()
                .enumerate()
                .filter_map(|(i, word)| line.rfind(word).map(|s_idx| (s_idx, i+1)))
                .max_by_key(|&(i, _)| i);

            let last_num = match (digit_find, word_find) {
                (Some((i, &b)), Some((wi, wv))) => {
                    if bs.len() - i - 1 > wi {
                        (b - b'0') as usize
                    } else {
                        wv
                    }
                }
                (Some((_, &b)), None) => (b - b'0') as usize,
                (None, Some((_, wv))) => wv,
                (None, None) => panic!("did not find word or digit in line"),
            };

            (first_num * 10 + last_num) as u32
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
