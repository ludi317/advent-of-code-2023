use hashbrown::HashSet;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let first_line = input.lines().next().unwrap();
    let win_start = first_line.find(":").unwrap() + 2;
    let win_end = first_line.find("|").unwrap() - 1;
    let have_start = win_end + 3;
    let sum = input
        .lines()
        .map(|line| {
            let win: HashSet<u32> = get_nums(&line[win_start..win_end]).into_iter().collect();
            let have = get_nums(&line[have_start..]);
            let num_matches = have.iter().filter(|&n| win.contains(n)).count();
            if num_matches > 1 {
                return 1 << (num_matches - 1);
            }
            num_matches
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.split('\n').collect();
    let first_line = lines[0];
    let win_start = first_line.find(":").unwrap() + 2;
    let win_end = first_line.find("|").unwrap() - 1;
    let have_start = win_end + 3;
    let mut card_count = vec![1u32; lines.len()+1];
    card_count[0] = 0;
    let mut i = 0;
    for line in lines {
        i += 1;
        let win: HashSet<u32> = get_nums(&line[win_start..win_end]).into_iter().collect();
        let have = get_nums(&line[have_start..]);
        let num_matches = have.iter().filter(|&n| win.contains(n)).count();
        for j in 1..=num_matches {
            card_count[i+j] += card_count[i];
        }

    }

    Some(card_count.iter().sum())
}

// parses a sequence of *non-negative* numbers
fn get_nums(s: &str) -> Vec<u32> {
    let mut nums = vec![];
    let mut num = 0;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap();
            num_found = true;
        } else if num_found {
            nums.push(num);
            num = 0;
            num_found = false;
        }
    }
    if num != 0 {
        nums.push(num);
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nums() {
        let s = " 4 3 5 645 23 2 0 77    3 ";
        let v = get_nums(s);
        assert_eq!(v, vec![4, 3, 5, 645, 23, 2, 0, 77,3]);

    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
