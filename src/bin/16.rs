advent_of_code::solution!(16);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn get_num(d: Direction) -> u32 {
    match d {
        Direction::Up => 0,
        Direction::Left => 1,
        Direction::Down => 2,
        Direction::Right => 3,
    }
}

fn get_delta(d: Direction) -> [isize; 2] {
    match d {
        Direction::Up => [-1, 0],
        Direction::Left => [0, -1],
        Direction::Down => [1, 0],
        Direction::Right => [0, 1],
    }
}

fn rotate(d: Direction, c: char) -> Vec<Direction> {
    match (d, c) {
        // no change
        (d, '.') => vec![d],
        (Direction::Up | Direction::Down, '|') => vec![d],
        (Direction::Left | Direction::Right, '-') => vec![d],
        // splitting
        (Direction::Up | Direction::Down, '-') => vec![Direction::Left, Direction::Right],
        (Direction::Left | Direction::Right, '|') => vec![Direction::Up, Direction::Down],
        // mirrors
        (Direction::Up, '\\') => vec![Direction::Left],
        (Direction::Left, '\\') => vec![Direction::Up],
        (Direction::Down, '\\') => vec![Direction::Right],
        (Direction::Right, '\\') => vec![Direction::Down],

        (Direction::Up, '/') => vec![Direction::Right],
        (Direction::Left, '/') => vec![Direction::Down],
        (Direction::Down, '/') => vec![Direction::Left],
        (Direction::Right, '/') => vec![Direction::Up],
        _ => unreachable!(),
    }
}

fn beam(
    grid: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<usize>>,
    mut pos: [isize; 2],
    mut d: Direction,
    visited: &mut Vec<Vec<u32>>,
) {
    let rows = grid.len();
    let cols = grid[0].len();

    while is_in_bounds(&pos, rows as isize, cols as isize) && is_not_visited(visited, &pos, d) {
        energized[pos[0] as usize][pos[1] as usize] = 1;
        visited[pos[0] as usize][pos[1] as usize] |= 1 << get_num(d);
        let ds = rotate(d, grid[pos[0] as usize][pos[1] as usize]);
        if ds.len() > 1 {
            let delta = get_delta(ds[1]);
            beam(
                grid,
                energized,
                [pos[0] + delta[0], pos[1] + delta[1]],
                ds[1],
                visited,
            );
        }
        d = ds[0];
        let delta = get_delta(d);
        pos[0] += delta[0];
        pos[1] += delta[1];
    }
}

fn is_not_visited(cache: &mut Vec<Vec<u32>>, pos: &[isize; 2], d: Direction) -> bool {
    cache[pos[0] as usize][pos[1] as usize] & (1 << get_num(d)) == 0
}

fn is_in_bounds(pos: &[isize; 2], rows: isize, cols: isize) -> bool {
    pos[0] >= 0 && pos[0] < rows && pos[1] >= 0 && pos[1] < cols
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut energized: Vec<Vec<usize>> = vec![vec![0usize; cols]; rows];
    let pos: [isize; 2] = [0, 0];
    let d = Direction::Right;
    let mut cache: Vec<Vec<u32>> = vec![vec![0u32; cols]; rows];
    beam(&grid, &mut energized, pos, d, &mut cache);

    let ans: usize = energized.iter().flat_map(|row| row.iter()).sum();
    Some(ans)
}

fn border_iter(rows: usize, cols: usize) -> impl Iterator<Item = ([usize; 2], Direction)> {
    let top_row = (0..cols - 1).map(|col| ([0, col], Direction::Down));
    let right_col = (0..rows - 1).map(move |row| ([row, cols - 1], Direction::Left));
    let bottom_row = (0..cols - 1).map(move |col| ([rows - 1, col], Direction::Up));
    let left_col = (0..rows - 1).map(|row| ([row, 0], Direction::Right));

    top_row.chain(right_col).chain(bottom_row).chain(left_col)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut ans: usize = 0;

    for (pos, d) in border_iter(rows, cols) {
        let pos: [isize; 2] = [pos[0] as isize, pos[1] as isize];
        let mut energized: Vec<Vec<usize>> = vec![vec![0usize; cols]; rows];
        let mut cache: Vec<Vec<u32>> = vec![vec![0u32; cols]; rows];
        beam(&grid, &mut energized, pos, d, &mut cache);

        ans = ans.max(energized.iter().flat_map(|row| row.iter()).sum());
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
