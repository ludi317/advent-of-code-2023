advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = text_to_grid(input);
    let mut part_num_sum = 0;
    for r in 0..grid.len() {
        let mut num: u32 = 0;
        let mut is_near_symbol = false;
        for c in 0..grid[0].len() {
            if grid[r][c].is_ascii_digit() {
                num = num * 10 + grid[r][c].to_digit(10).unwrap();
                is_near_symbol = is_near_symbol || is_next_to_symbol(&grid, r, c);
            } else {
                if is_near_symbol {
                    part_num_sum += num;
                }
                num = 0;
                is_near_symbol = false;
            }
        }
        // end of row
        if is_near_symbol {
            part_num_sum += num;
        }
    }

    Some(part_num_sum)
}

fn is_next_to_symbol(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    for r in i.saturating_sub(1)..=(i + 1).min(grid.len() - 1) {
        for c in j.saturating_sub(1)..=(j + 1).min(grid[0].len() - 1) {
            if (r == i) && (c == j) {
                continue;
            }
            let neigh: char = grid[r][c];
            // symbol
            if !neigh.is_ascii_digit() && neigh != '.' {
                return true;
            }
        }
    }
    false
}

fn text_to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = text_to_grid(input);
    let mut gear_ratio_sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '*' {
                let mut num_count = 0;
                let mut prod = 1;
                for i in r.saturating_sub(1)..=(r + 1).min(grid.len() - 1) {
                    let mut j = c.saturating_sub(1);
                    while j <= (c + 1).min(grid[0].len()) {
                        // found a number
                        if grid[i][j].is_ascii_digit() {
                            num_count += 1;
                            // move j rightwards to avoid double-counting long numbers
                            let mut num = 0;
                            (num, j) = expand(&grid, i, j);
                            prod *= num;
                        } else {
                            j += 1;
                        }
                    }
                }
                if num_count == 2 {
                    gear_ratio_sum += prod;
                }
            }
        }
    }
    Some(gear_ratio_sum)
}

fn expand(grid: &Vec<Vec<char>>, r: usize, c: usize) -> (u32, usize) {
    let row = &grid[r];
    let mut j: i8 = c as i8;
    // move leftward
    while j >= 0 && row[j as usize].is_ascii_digit() {
        j -= 1;
    }
    j += 1;
    let mut j = j as usize;
    let mut num = 0;
    // parse the number from left to right
    while j < row.len() && row[j].is_ascii_digit() {
        num = 10 * num + row[j].to_digit(10).unwrap();
        j += 1;
    }
    (num, j)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
