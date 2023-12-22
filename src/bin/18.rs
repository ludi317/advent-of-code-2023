use advent_of_code::get_nums;
advent_of_code::solution!(18);

#[derive(Debug)]
struct Trench {
    dir: usize,
    steps: isize,
}

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

const DIRS: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

pub fn part_one(input: &str) -> Option<isize> {
    let trenches: Vec<Trench> = input
        .lines()
        .map(|line| {
            let steps = get_nums(&line[2..4])[0];
            let dir = match &line[0..1].as_bytes()[0] {
                b'U' => UP,
                b'R' => RIGHT,
                b'D' => DOWN,
                b'L' => LEFT,
                _ => unreachable!(),
            };
            Trench { steps, dir }
        })
        .collect();

    Some(trench_count(trenches))
}

fn trench_count(trenches: Vec<Trench>) -> isize {
    let mut pos = [0isize, 0isize];
    let mut double_area = 0;
    let mut boundary = 0;
    for t in trenches {
        let next_pos = [
            pos[0] + DIRS[t.dir][0] * t.steps,
            pos[1] + DIRS[t.dir][1] * t.steps,
        ];
        // Shoelace formula for area of polygon given its points:
        // https://en.wikipedia.org/wiki/Shoelace_formula
        double_area += determinant(pos, next_pos);
        pos = next_pos;
        boundary += t.steps;
    }

    // Pick's theorem for number of interior points given a polygon's area and number of boundary points
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b/2 - 1
    // i = A - b/2 + 1
    let interior = double_area.abs() / 2 - boundary / 2 + 1;
    interior + boundary
}

fn determinant(a: [isize; 2], b: [isize; 2]) -> isize {
    a[0] * b[1] - a[1] * b[0]
}

pub fn part_two(input: &str) -> Option<isize> {
    let trenches: Vec<Trench> = input
        .lines()
        .map(|line| {
            let hash_pos = line.find("#").unwrap();
            let steps =
                u32::from_str_radix(&line[hash_pos + 1..hash_pos + 6], 16).unwrap() as isize;
            let dir = match &line[hash_pos + 6..hash_pos + 7].as_bytes()[0] {
                b'3' => UP,
                b'0' => RIGHT,
                b'1' => DOWN,
                b'2' => LEFT,
                _ => unreachable!(),
            };
            Trench { steps, dir }
        })
        .collect();
    Some(trench_count(trenches))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
