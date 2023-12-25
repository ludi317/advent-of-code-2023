use hashbrown::HashSet;
use std::collections::VecDeque;
advent_of_code::solution!(21);

const STEPS: usize = 6;

const DIRS: [[isize; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

pub fn part_one(input: &str) -> Option<u32> {
    let mut start = [0isize, 0isize];
    let mut grid: Vec<Vec<char>> = vec![];
    for (r, line) in input.lines().enumerate() {
        if let Some(idx) = line.find('S') {
            start = [r as isize, idx as isize];
        }
        grid.push(line.chars().collect());
    }

    let mut q = VecDeque::new();
    let mut seen: HashSet<[isize; 2]> = HashSet::new();

    q.push_back(start);
    for _ in 1..=STEPS {
        seen.clear();
        let length = q.len();
        for _ in 0..length {
            let n = q.pop_front().unwrap();
            for dir in DIRS {
                let nr = dir[0] + n[0];
                let nc = dir[1] + n[1];
                if nr == -1
                    || nr == grid.len() as isize
                    || nc == -1
                    || nc == grid[0].len() as isize
                    || grid[nr as usize][nc as usize] == '#'
                    || seen.contains(&[nr, nc])
                {
                    continue;
                }
                seen.insert([nr, nc]);
                q.push_back([nr, nc]);
            }
        }
    }

    Some(seen.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start = [0isize, 0isize];
    let mut grid: Vec<Vec<char>> = vec![];
    for (r, line) in input.lines().enumerate() {
        if let Some(idx) = line.find('S') {
            start = [r as isize, idx as isize];
        }
        grid.push(line.chars().collect());
    }

    let mut q = VecDeque::new();
    let mut seen: HashSet<[isize; 2]> = HashSet::new();
    let r = grid.len() as isize;
    let c = grid[0].len() as isize;

    q.push_back(start);
    for _ in 1..=STEPS {
        seen.clear();
        let length = q.len();
        for _ in 0..length {
            let n = q.pop_front().unwrap();
            for dir in DIRS {
                let nr = dir[0] + n[0];
                let nc = dir[1] + n[1];
                if grid[(((nr % r) + r) % r) as usize][(((nc % c) + c) % c) as usize] == '#'
                    || seen.contains(&[nr, nc])
                {
                    continue;
                }
                seen.insert([nr, nc]);
                q.push_back([nr, nc]);
            }
        }
    }
    Some(seen.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
