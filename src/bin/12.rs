use advent_of_code::get_nums_usize;
advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input
        .lines()
        .map(|line| {
            let space_idx = line.find(' ').unwrap();
            let haystack = &line[..space_idx];
            let hash_counts = get_nums_usize(&line[space_idx..]);
            possible_arrangements(haystack.as_bytes(), &hash_counts)
        })
        .sum();
    Some(sum)
}

pub fn possible_arrangements(springs: &[u8], damaged: &[usize]) -> usize {
    // Setup the DP table
    let num_springs = springs.len();
    let num_damaged = damaged.len();

    // [position][group][count]
    let mut dp = vec![vec![vec![0; num_springs + 1]; num_damaged + 1]; num_springs + 1];

    // Base values

    // set dp[0..=p+1][0][0] to 1 while springs[..=p] is all dots
    dp[0][0][0] = 1;
    for p in 0..springs.len() {
        if matches!(springs[p], b'?' | b'.') {
            dp[p + 1][0][0] = 1
        } else {
            break;
        }
    }
    // set all remaining damaged springs for first group
    dp[0][1][damaged[0]] = 1;

    // Go through the conditions
    for position in 0..num_springs {
        for (group, &j) in damaged.iter().enumerate() {
            for rem_d_springs in 0..=j {
                for &ch in &[b'.', b'#'] {
                    // If the current character is not a '?' or the character type
                    // we're looking for, then we can skip it.
                    if ![ch, b'?'].contains(&springs[position]) {
                        continue;
                    }

                    // 3 transitions
                    let value = match (ch, rem_d_springs) {
                        (b'.', 0) => dp[position][group + 1][0], // finished group, carry current completed group
                        (b'.', all_) if damaged[group] == all_ => dp[position][group][0], // starting new group, carry previous completed group
                        (b'#', _) => dp[position][group + 1][rem_d_springs + 1], // consume a remaining damaged spring
                        _ => 0,
                    };

                    dp[position + 1][group + 1][rem_d_springs] += value;
                }
            }
        }
    }

    dp[num_springs][num_damaged][0]
}

pub fn part_two(input: &str) -> Option<usize> {
    let sum = input
        .lines()
        .map(|line| {
            let space_idx = line.find(' ').unwrap();
            let haystack = std::iter::repeat(&line[..space_idx])
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let hash_counts = get_nums_usize(&line[space_idx..]).repeat(5);
            possible_arrangements(haystack.as_bytes(), &hash_counts)
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
        assert_eq!(result, Some(21));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(1));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
