advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let blank_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|&(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect();

    let blank_cols: Vec<usize> = (0..grid[0].len())
        .filter(|&c| grid.iter().all(|row| row[c] == '.'))
        .collect();

    let mut galaxies: Vec<[usize; 2]> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.push([i, j]);
            }
        }
    }

    let mut cum_sum = vec![0; galaxies.len() + 1];
    // expand rows
    let mut i = galaxies.len() - 1;
    for b in blank_rows.iter().rev() {
        while i >= 0 && galaxies[i][0] > *b {
            i -= 1;
        }
        i += 1;
        cum_sum[i] += 1;
    }

    let mut sum: usize = 0;
    for i in 0..galaxies.len() {
        sum += cum_sum[i];
        galaxies[i][0] += sum;
    }

    // expand cols
    galaxies.sort_by(|a, b| a[1].cmp(&b[1]));

    cum_sum.fill(0);

    let mut i = galaxies.len() - 1;
    for b in blank_cols.iter().rev() {
        while i >= 0 && galaxies[i][1] > *b {
            i -= 1;
        }
        i += 1;

        cum_sum[i] += 1;
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        sum += cum_sum[i];
        galaxies[i][1] += sum;
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

pub fn part_two(input: &str) -> Option<usize> {
    const EXP: u32 = 1_000_000;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let blank_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|&(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect();

    let blank_cols: Vec<usize> = (0..grid[0].len())
        .filter(|&c| grid.iter().all(|row| row[c] == '.'))
        .collect();

    let mut galaxies: Vec<[usize; 2]> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.push([i, j]);
            }
        }
    }

    let mut cum_sum = vec![0; galaxies.len() + 1];
    // expand rows
    let mut i = galaxies.len() - 1;
    for b in blank_rows.iter().rev() {
        while i >= 0 && galaxies[i][0] > *b {
            i -= 1;
        }
        i += 1;
        cum_sum[i] += (EXP - 1) as usize;
    }

    let mut sum: usize = 0;
    for i in 0..galaxies.len() {
        sum += cum_sum[i];
        galaxies[i][0] += sum;
    }

    // expand cols
    galaxies.sort_by(|a, b| a[1].cmp(&b[1]));

    cum_sum.fill(0);

    let mut i = galaxies.len() - 1;
    for b in blank_cols.iter().rev() {
        while i >= 0 && galaxies[i][1] > *b {
            i -= 1;
        }
        i += 1;

        cum_sum[i] += (EXP - 1) as usize;
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        sum += cum_sum[i];
        galaxies[i][1] += sum;
    }

    let mut sum_ans = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dist =
                galaxies[j][0].abs_diff(galaxies[i][0]) + galaxies[j][1].abs_diff(galaxies[i][1]);
            sum_ans += dist
        }
    }
    Some(sum_ans)
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
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(8410));
    }
}
