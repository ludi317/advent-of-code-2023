advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    calc_dist(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    calc_dist(input, 100)
}

fn calc_dist(input: &str, exp_factor: usize) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // row-based expansion
    let mut galaxies = vec![];
    let mut num_prev_blanks = 0;
    for i in 0..grid.len() {
        let mut num_galaxies_in_row = 0;
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                galaxies.push([i, j]);
                num_galaxies_in_row += 1;
            }
        }
        if num_galaxies_in_row == 0 {
            num_prev_blanks += 1;
        } else {
            for i in 0..num_galaxies_in_row {
                let g_len = galaxies.len();
                galaxies[g_len - 1 - i][0] += num_prev_blanks * (exp_factor - 1)
            }
        }
    }

    // sort galaxies by col
    galaxies.sort_by(|a, b| a[1].cmp(&b[1]));

    // col-based expansion
    num_prev_blanks = 0;
    let mut galaxy_idx = 0;
    for j in 0..grid[0].len() {
        let mut num_galaxies_in_col = 0;
        for i in 0..grid.len() {
            if grid[i][j] == '#' {
                num_galaxies_in_col += 1;
            }
        }
        if num_galaxies_in_col == 0 {
            num_prev_blanks += 1;
        } else {
            for i in 0..num_galaxies_in_col {
                galaxies[galaxy_idx + i][1] += num_prev_blanks * (exp_factor - 1);
            }
            galaxy_idx += num_galaxies_in_col;
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dist =
                galaxies[j][0].abs_diff(galaxies[i][0]) + galaxies[j][1].abs_diff(galaxies[i][1]);
            sum += dist
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        // Set EXP to 100
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
