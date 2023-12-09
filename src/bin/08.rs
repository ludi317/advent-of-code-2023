use hashbrown::HashMap;
use num::integer::lcm;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut lines = input.lines();
    let instrs = lines.next().unwrap();
    lines.next(); // skip blank line
    for line in lines {
        map.insert(&line[..3], [&line[7..10], &line[12..15]]);
    }

    let mut count = 0;
    let mut cur = "AAA";
    'outerLoop: loop {
        for dir in instrs.chars() {
            let idx = { if dir == 'L' {0} else {1}};
            cur = map.get(cur).unwrap()[idx];
            count += 1;
            if cur == "ZZZ" {
                break 'outerLoop;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut lines = input.lines();
    let instrs = lines.next().unwrap();
    lines.next(); // skip blank line
    let mut nodes: Vec<&str> = vec![];
    for line in lines {
        let from = &line[..3];
        map.insert(from, [&line[7..10], &line[12..15]]);
        if from.ends_with('A') {
            nodes.push(from);
        }
    }

    let mut count = 0;
    let mut cycle_lengths = vec![0; nodes.len()];
    let mut num_found = 0;
    loop {
        for dir in instrs.chars() {
            let idx = { if dir == 'L' {0} else {1}};
            // step
            for node in nodes.iter_mut() {
                *node = map.get(node).unwrap()[idx];
            }
            count += 1;
            for (i, node) in nodes.iter().enumerate() {
                if cycle_lengths[i] == 0 && node.ends_with('Z') {
                    num_found += 1;
                    cycle_lengths[i] = count;
                }
            }
            if num_found == nodes.len() {
                // compute lcm
                let lcm_result = cycle_lengths.iter().fold(1, |acc, &num| lcm(acc, num));
                return Some(lcm_result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}
