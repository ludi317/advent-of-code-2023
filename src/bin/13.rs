advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let mut pattern: Vec<Vec<char>> = vec![];
    let mut tot = 0;
    for line in input.lines() {
        if line.is_empty() {
            // process pattern
            tot += process_pattern(&pattern);
            pattern.clear();
        } else {
            pattern.push(line.chars().collect());
        }
    }

    // process last pattern
    tot += process_pattern(&pattern);
    Some(tot)
}

fn process_pattern(pattern: &Vec<Vec<char>>) -> usize {
    let mut ans: usize = 0;
    for i in 1..pattern.len() {
        // expand
        let mut top = (i - 1) as isize;
        let mut bottom = i;
        while top >= 0 && bottom < pattern.len() && pattern[top as usize] == pattern[bottom] {
            top -= 1;
            bottom += 1;
        }
        if top == -1 || bottom == pattern.len() {
            ans += 100* i;
        }
    }

    for i in 1..pattern[0].len() {
        // expand
        let mut left = (i - 1) as isize;
        let mut right = i;
        while left >= 0
            && right < pattern[0].len()
            && pattern.iter().all(|row| row[left as usize] == row[right])
        {
            left -= 1;
            right += 1;
        }
        if left == -1 || right == pattern[0].len() {
            ans += i;
        }
    }

    ans
}


fn process_pattern2(pattern: &Vec<Vec<char>>) -> usize {
    let mut ans: usize = 0;
    for i in 1..pattern.len() {
        // expand
        let mut top = (i - 1) as isize;
        let mut bottom = i;
        let mut diff_count = 0;
        while top >= 0 && bottom < pattern.len() {
            if pattern[top as usize] != pattern[bottom] {
                diff_count += pattern[top as usize]
                    .iter()
                    .zip(pattern[bottom].iter())
                    .filter(|(&c1, &c2)| c1 != c2)
                    .count();
            }
            if diff_count > 1 {
                break;
            }
            top -= 1;
            bottom += 1;
        }
        if (top == -1 || bottom == pattern.len()) && diff_count == 1 {
            ans += 100 * i;
        }
    }

    for i in 1..pattern[0].len() {
        // expand
        let mut left = (i - 1) as isize;
        let mut right = i;
        let mut diff_count = 0;
        while left >= 0 && right < pattern[0].len() {
            diff_count += pattern
                .iter()
                .map(|row| (row[left as usize] != row[right]) as usize)
                .sum::<usize>();
            if diff_count > 1 {
                break;
            }
            left -= 1;
            right += 1;
        }
        if (left == -1 || right == pattern[0].len()) && diff_count == 1 {
            ans += i;
        }
    }

    ans
}


pub fn part_two(input: &str) -> Option<usize> {
    let mut pattern: Vec<Vec<char>> = vec![];
    let mut tot = 0;
    for line in input.lines() {
        if line.is_empty() {
            // process pattern
            tot += process_pattern2(&pattern);
            pattern.clear();
        } else {
            pattern.push(line.chars().collect());
        }
    }

    // process last pattern
    tot += process_pattern2(&pattern);
    Some(tot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
