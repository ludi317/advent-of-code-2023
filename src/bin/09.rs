use advent_of_code::get_nums;
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let sum = input
        .lines()
        .map(|line| {
            let mut v: Vec<Vec<isize>> = vec![];
            v.push(get_nums(line));
            // while last line is not all zeroes
            while !v[v.len() - 1].iter().all(|&e| e == 0) {
                let mut v_next = vec![];
                let last_vec = &v[v.len() - 1];
                for i in 0..(last_vec.len() - 1) {
                    // take the diff
                    v_next.push(last_vec[i + 1] - last_vec[i])
                }
                v.push(v_next);
            }
            let last_idx = v.len() - 1;
            v[last_idx].push(0);
            for i in (0..=v.len() - 2).rev() {
                let left = v[i][v[i].len() - 1];
                let down = v[i + 1][v[i + 1].len() - 1];
                v[i].push(left + down);
            }
            v[0][v[0].len() -1]
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<isize> {
    let sum = input
        .lines()
        .map(|line| {
            let mut v: Vec<Vec<isize>> = vec![];
            let mut nums = get_nums(line);
            nums.reverse();
            v.push(nums);
            // while last line is not all zeroes
            while !v[v.len() - 1].iter().all(|&e| e == 0) {
                let mut v_next = vec![];
                let last_vec = &v[v.len() - 1];
                for i in 0..(last_vec.len() - 1) {
                    // take the diff
                    v_next.push(last_vec[i + 1] - last_vec[i])
                }
                v.push(v_next);
            }
            let last_idx = v.len() - 1;
            v[last_idx].push(0);
            for i in (0..=v.len() - 2).rev() {
                let left = v[i][v[i].len() - 1];
                let down = v[i + 1][v[i + 1].len() - 1];
                v[i].push(left + down);
            }
            v[0][v[0].len() -1]
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
