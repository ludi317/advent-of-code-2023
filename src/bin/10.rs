use hashbrown::HashSet;
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: [isize; 2] = [0, 0];
    let grid: Vec<Vec<char>> = input
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

    let mut cur_dir = 0;
    let dirs: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let is_connecting_tile = ["|7F", "-7J", "|JL", "-FL"];

    // find first dir from S
    for (i, dir) in dirs.iter().enumerate() {
        let nr = start[0] + dir[0];
        let nc = start[1] + dir[1];
        if nr == -1 || nr == rows || nc == -1 || nc == cols {
            continue;
        }
        if is_connecting_tile[i].contains(grid[nr as usize][nc as usize]) {
            cur_dir = i;
        }
    }
    // move once
    let mut pos = [dirs[cur_dir][0] + start[0], dirs[cur_dir][1] + start[1]];
    let mut path_len = 1;
    while pos != start {
        let c = grid[pos[0] as usize][pos[1] as usize];
        cur_dir = match (cur_dir, c) {
            (0, '|') => 0,
            (0, '7') => 3,
            (0, 'F') => 1,
            (1, '-') => 1,
            (1, '7') => 2,
            (1, 'J') => 0,
            (2, '|') => 2,
            (2, 'J') => 3,
            (2, 'L') => 1,
            (3, '-') => 3,
            (3, 'L') => 0,
            (3, 'F') => 2,
            _ => unreachable!(),
        };

        pos[0] += dirs[cur_dir][0];
        pos[1] += dirs[cur_dir][1];

        path_len += 1;
    }

    Some(path_len / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start: [isize; 2] = [0, 0];
    let grid: Vec<Vec<char>> = input
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

    let mut cur_dir = 0;
    let dirs: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let is_connecting_tile = ["|7F", "-7J", "|JL", "-FL"];
    let mut is_S_IJL = false;

    // brute force find first dir from S
    for (i, dir) in dirs.iter().enumerate() {
        let nr = start[0] + dir[0];
        let nc = start[1] + dir[1];
        if nr == -1 || nr == rows || nc == -1 || nc == cols {
            continue;
        }
        if is_connecting_tile[i].contains(grid[nr as usize][nc as usize]) {
            cur_dir = i;
            is_S_IJL = i == 0; // if up is a valid direction, S must be |, J, or L
            break;
        }
    }
    let mut path: HashSet<[isize; 2]> = HashSet::new();
    let mut pos = [dirs[cur_dir][0] + start[0], dirs[cur_dir][1] + start[1]];
    path.insert(pos);
    while pos != start {
        let c = grid[pos[0] as usize][pos[1] as usize];
        cur_dir = match (cur_dir, c) {
            (0, '|') => 0,
            (0, '7') => 3,
            (0, 'F') => 1,
            (1, '-') => 1,
            (1, '7') => 2,
            (1, 'J') => 0,
            (2, '|') => 2,
            (2, 'J') => 3,
            (2, 'L') => 1,
            (3, '-') => 3,
            (3, 'L') => 0,
            (3, 'F') => 2,
            _ => cur_dir,
        };

        pos[0] += dirs[cur_dir][0];
        pos[1] += dirs[cur_dir][1];
        path.insert(pos);
    }

    // count the number of tiles on the inside of the path
    // Clearly, crossing a vertical boundary changes whether you are inside or out.
    // There are four corners: 'L', 'J', 'F', and '7'.
    // Scanning rows from left to right gives an 'L' or 'F' first. The path loops within the grid.
    // 'L' can be followed by '-'* and either '7' or 'J'.
    // Same for 'F'.
    // An 'L-*J' should flip the inside flag an even number of times. That is, if you were on the outside (or inside) before the L, you remain on the outside (or inside) after the J.
    // Same for 'F-*7'.
    // 'L-*7' should flip the inside flag and odd number of times. It's like crossing vertical boundary. Outside -> Inside and Inside -> Outside.
    // Same for 'F-*J'.
    // L + J = 0 or 2
    // F + 7 = 0 or 2
    // L + 7 = 1
    // F + J = 1
    // Solve and get either:
    // L = 1, J = 1, F = 0, 7 = 0
    // L = 0, J = 0, F = 1, 7 = 1
    let mut inside_count = 0;
    for i in 0..rows {
        let mut inside = false;
        for j in 0..cols {
            if path.contains(&[i, j]) {
                let c = grid[i as usize][j as usize];
                if matches!(c, '|' | 'J' | 'L') || (c == 'S' && is_S_IJL) {
                    inside = !inside;
                }
            } else {
                if inside {
                    inside_count += 1;
                }
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
