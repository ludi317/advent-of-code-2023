use hashbrown::HashSet;
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: [isize; 2] = [0, 0];
    let grid: Vec<Vec<char>> = input.lines().enumerate().map(|(i, line)| {
            // find the start
            if let Some(j) = line.find('S') {
                start = [i as isize, j as isize];
            }
            line.chars().collect()
        })
        .collect();
    let cols = grid[0].len() as isize;
    let rows = grid.len() as isize;

    let mut first_pos = [0, 0];
    let dirs: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

    // find first tile from S
    for (i, dir) in dirs.iter().enumerate() {
        let nr = start[0] + dir[0];
        let nc = start[1] + dir[1];
        if nr == -1 || nr == rows || nc == -1 || nc == cols {
            continue;
        }
        if is_connecting_tile(grid[nr as usize][nc as usize], i) {
            first_pos = [nr, nc];
            break;
        }
    }
    let mut prev = start;
    let mut pos = first_pos;
    let mut path_len = 1;
    while pos != start {
        let delta = match grid[pos[0] as usize][pos[1] as usize] {
            '|' => [[1, 0], [-1, 0]],
            '-' => [[0, 1], [0, -1]],
            'L' => [[-1, 0], [0, 1]],
            'J' => [[-1, 0], [0, -1]],
            '7' => [[1, 0], [0, -1]],
            'F' => [[1, 0], [0, 1]],
            _ => unreachable!(),
        };

        for d in delta {
            let new_pos = [d[0] + pos[0], d[1] + pos[1]];
            // don't go back to the previous tile
            if new_pos != prev {
                prev = pos;
                pos = new_pos;
                break;
            }
        }
        path_len += 1;
    }

    Some(path_len / 2)
}

fn is_connecting_tile(tile: char, dir_idx: usize) -> bool {
    match dir_idx {
        0 => matches!(tile, '|' | '7' | 'F'), // north
        1 => matches!(tile, '-' | 'J' | '7'), // east
        2 => matches!(tile, '|' | 'L' | 'J'), // south
        3 => matches!(tile, '-' | 'L' | 'F'), // west
        _ => false,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start: [isize; 2] = [0, 0];
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            // find the start
            if let Some(j) = line.find('S') {
                start = [i as isize, j as isize];
            }
            line.chars().collect()
        })
        .collect();
    let cols = grid[0].len() as isize;
    let rows = grid.len() as isize;

    let mut first_pos = [0, 0];
    let dirs: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

    // find first tile from S
    for (i, dir) in dirs.iter().enumerate() {
        let nr = start[0] + dir[0];
        let nc = start[1] + dir[1];
        if nr == -1 || nr == rows || nc == -1 || nc == cols {
            continue;
        }
        if is_connecting_tile(grid[nr as usize][nc as usize], i) {
            first_pos = [nr, nc];
            break;
        }
    }
    let mut path: HashSet<[isize; 2]> = HashSet::new();
    path.insert(first_pos);
    let mut prev = start;
    let mut pos = first_pos;
    while pos != start {
        let delta = match grid[pos[0] as usize][pos[1] as usize] {
            '|' => [[1, 0], [-1, 0]],
            '-' => [[0, 1], [0, -1]],
            'L' => [[-1, 0], [0, 1]],
            'J' => [[-1, 0], [0, -1]],
            '7' => [[1, 0], [0, -1]],
            'F' => [[1, 0], [0, 1]],
            _ => unreachable!(),
        };

        for d in delta {
            let new_pos = [d[0] + pos[0], d[1] + pos[1]];
            // don't go back to the previous tile
            if new_pos != prev {
                prev = pos;
                pos = new_pos;
                break;
            }
        }
        path.insert(pos);
    }

    // replace S with a pipe

    // find its neighbors in the path
    let mut neighs = [0usize; 2];
    let mut j = 0;
    for (i, dir) in dirs.iter().enumerate() {
        if path.contains(&[dir[0] + start[0], dir[1] + start[1]])
            && is_connecting_tile(grid[(dir[0] + start[0]) as usize][(dir[1] + start[1]) as usize], i)
        {
            neighs[j] = i;
            j += 1;
        }
    }

    // replace S
    grid[start[0] as usize][start[1] as usize] = match neighs {
        [0, 1] => 'L',
        [0, 2] => '|',
        [0, 3] => 'J',
        [1, 2] => 'F',
        [1, 3] => '-',
        [2, 3] => '7',
        _ => unreachable!(),
    };

    // count
    let mut inside_count = 0;
    let mut inside = false;
    for i in 0..rows {
        for j in 0..cols {
            if path.contains(&[i, j]) && matches!(grid[i as usize][j as usize], '|' | 'J' | 'L') {
                inside = !inside;
            }
            if inside && !path.contains(&[i, j]) {
                inside_count += 1;
            }
        }
    }

    Some(inside_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }
}
