use advent_of_code::get_nums;
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let sum = input
        .lines()
        .map(|line| {
            let mut nums = get_nums(line);
            let mut ends = nums[nums.len() - 1];
            // while nums is not all zeroes
            while !nums.iter().all(|&e| e == 0) {
                nums = nums.windows(2).map(|pair| pair[1] - pair[0]).collect();
                ends += nums[nums.len() - 1];
            }
            ends
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<isize> {
    let sum = input
        .lines()
        .map(|line| {
            let mut nums = get_nums(line);
            nums.reverse();
            let mut ends = nums[nums.len() - 1];
            // while nums is not all zeroes
            while !nums.iter().all(|&e| e == 0) {
                nums = nums.windows(2).map(|pair| pair[1] - pair[0]).collect();
                ends += nums[nums.len() - 1];
            }
            ends
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
