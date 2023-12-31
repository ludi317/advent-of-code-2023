use hashbrown::HashSet;
use std::collections::VecDeque;
advent_of_code::solution!(21);

const DIRS: [[isize; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start) = parse(input);
    Some(bfs(&grid, start, 6))
}

fn parse(input: &str) -> (Vec<Vec<char>>, [isize; 2]) {
    let mut start = [0isize, 0isize];
    let mut grid: Vec<Vec<char>> = vec![];
    for (r, line) in input.lines().enumerate() {
        if let Some(idx) = line.find('S') {
            start = [r as isize, idx as isize];
        }
        grid.push(line.chars().collect());
    }
    (grid, start)
}

fn bfs(grid: &Vec<Vec<char>>, start: [isize; 2], steps: usize) -> usize {
    let r: isize = grid.len() as isize;
    let c: isize = grid[0].len() as isize;
    let mut q = VecDeque::new();
    let mut seen: HashSet<[isize; 2]> = HashSet::new();

    q.push_back(start);
    for _ in 1..=steps {
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

    seen.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    // The input is a square grid with width 'w' and S in the center.
    // When steps 'n' is w/2, the grid is "completed". Same for every i*w + w/2 steps.
    // The requested steps, 26501365, just so happens to be w/2 mod w.
    // For these special steps, the number of reachable plots (and answer to this question) follows a quadratic formula.
    // Use polynomial interpolation to find the coefficients of the quadratic.
    let n = 26501365;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let w = grid.len();
    assert_eq!(w, grid[0].len(), "input must be a square");
    assert_eq!(
        w / 2,
        n % w,
        "number of steps mod width must be a half-width"
    );
    // use bfs to calculate num of steps for 3 smallest values where x mod w = w/2.
    let mut x = vec![0, 1, 2];
    for i in 0..x.len() {
        x[i] = (x[i] * w) + w / 2;
    }
    let (grid, start) = parse(input);
    let mut y: Vec<usize> = vec![];
    for i in 0..3 {
        y.push(bfs(&grid, start, x[i]));
    }
    // interpolate quadratic

    // y(i) = a * i^2 + b * i + c
    // y(0) = c
    let c = y[0];
    // y(1) = a + b + c
    // 2 * y(1) = 2*a + 2*b + 2*c
    // y(2) = 4*a + 2*b + c
    // -2*y(1) = -2*a -2*b - 2* c
    // y(2) - 2*y(1) = 2*a - c
    // (y(2) - 2 *y(1) + c)/2 = a
    let a = (y[2] - 2 * y[1] + c) / 2;
    let b = y[1] - a - c;

    let p = |v| a * v * v + b * v + c;

    let inv = |v| (v - (n % w)) / w;

    Some(p(inv(n)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
