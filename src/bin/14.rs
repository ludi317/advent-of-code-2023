use hashbrown::HashMap;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut rocks: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    tilt_north(&mut rocks);
    Some(calc_north_load(&rocks))
}

fn calc_north_load(rocks: &Vec<Vec<char>>) -> usize {
    rocks
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (rocks.len() - i))
        .sum()
}

fn tilt_east(rocks: &mut Vec<Vec<char>>) {
    let cols = rocks[0].len();
    let rows = rocks.len();
    for r in 0..rows {
        let mut num_round_rocks = 0;
        for c in 0..cols {
            if rocks[r][c] == '#' {
                // dump round rocks
                for i in 0..num_round_rocks {
                    rocks[r][c - i - 1] = 'O';
                }
                num_round_rocks = 0;
            } else if rocks[r][c] == 'O' {
                // collect round rocks
                rocks[r][c] = '.';
                num_round_rocks += 1;
            }
        }
        // dump round rocks
        for i in 0..num_round_rocks {
            rocks[r][cols - i - 1] = 'O';
        }
    }
}

fn tilt_south(rocks: &mut Vec<Vec<char>>) {
    let cols = rocks[0].len();
    let rows = rocks.len();
    for c in 0..cols {
        let mut num_round_rocks = 0;
        for r in 0..rows {
            if rocks[r][c] == '#' {
                // dump round rocks
                for i in 0..num_round_rocks {
                    rocks[r - i - 1][c] = 'O';
                }
                num_round_rocks = 0;
            } else if rocks[r][c] == 'O' {
                // collect round rocks
                rocks[r][c] = '.';
                num_round_rocks += 1;
            }
        }
        // dump round rocks
        for i in 0..num_round_rocks {
            rocks[rows - 1 - i][c] = 'O';
        }
    }
}

fn tilt_west(rocks: &mut Vec<Vec<char>>) {
    let cols = rocks[0].len();
    let rows = rocks.len();
    for r in 0..rows {
        let mut num_round_rocks = 0;
        for c in (0..cols).rev() {
            if rocks[r][c] == '#' {
                // dump round rocks
                for i in 0..num_round_rocks {
                    rocks[r][c + i + 1] = 'O';
                }
                num_round_rocks = 0;
            } else if rocks[r][c] == 'O' {
                // collect round rocks
                rocks[r][c] = '.';
                num_round_rocks += 1;
            }
        }
        // dump round rocks
        for i in 0..num_round_rocks {
            rocks[r][i] = 'O';
        }
    }
}

fn tilt_north(rocks: &mut Vec<Vec<char>>) {
    let cols = rocks[0].len();
    let rows = rocks.len();
    for c in 0..cols {
        let mut num_round_rocks = 0;
        for r in (0..rows).rev() {
            if rocks[r][c] == '#' {
                // dump round rocks
                for i in 0..num_round_rocks {
                    rocks[r + i + 1][c] = 'O';
                }
                num_round_rocks = 0;
            } else if rocks[r][c] == 'O' {
                // collect round rocks
                rocks[r][c] = '.';
                num_round_rocks += 1;
            }
        }
        // dump round rocks
        for i in 0..num_round_rocks {
            rocks[i][c] = 'O';
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let num_cycles = 1_000_000_000;
    let mut rocks: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut m: HashMap<usize, usize> = HashMap::new();
    let mut prev_delta = 0;
    let mut cycle_len = None;
    for i in 0..num_cycles {
        tilt_north(&mut rocks);
        tilt_west(&mut rocks);
        tilt_south(&mut rocks);
        tilt_east(&mut rocks);
        let north_load = calc_north_load(&rocks);
        // poor man's cycle detection
        if let Some(prev_idx) = m.get(&north_load) {
            let delta = i + 1 - prev_idx;
            if delta != 1 && prev_delta == delta {
                cycle_len = Some(delta);
            }
            prev_delta = delta;
        }
        if let Some(c) = cycle_len {
            if (i + 1) % c == num_cycles % c {
                return Some(north_load);
            }
        }

        m.insert(north_load, i + 1);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
