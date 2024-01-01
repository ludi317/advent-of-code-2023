advent_of_code::solution!(22);
use std::collections::{HashMap, HashSet, VecDeque};

// Solution taken from https://github.com/rene-d/advent-of-rust/commit/4789a1b85af107e6b1dced19819a0da8cda6f723

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(c: &[i32]) -> Self {
        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

struct Brick {
    a: Point,
    b: Point,
}

impl Brick {
    fn new(coords: &[i32]) -> Self {
        Self {
            a: Point::new(&coords[0..3]),
            b: Point::new(&coords[3..6]),
        }
    }

    /// Return true if two bricks overlap in 2D
    fn overlap(&self, other: &Self) -> bool {
        self.a.x.max(other.a.x) <= self.b.x.min(other.b.x)
            && self.a.y.max(other.a.y) <= self.b.y.min(other.b.y)
    }
}

struct Puzzle {
    bricks: Vec<Brick>,                           // list of bricks sorted lowest first
    supports: HashMap<usize, HashSet<usize>>,     // set of bricks supported by another brick
    supported_by: HashMap<usize, HashSet<usize>>, // set of bricks that support another brick
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            bricks: vec![],
            supports: HashMap::new(),
            supported_by: HashMap::new(),
        }
    }

    fn configure(&mut self, input: &str) {
        // load the bricks of sand
        for line in input.lines() {
            let coords: Vec<_> = line
                .split([',', '~'])
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            self.bricks.push(Brick::new(&coords));
        }

        // let a.z the lowest coordinate
        self.bricks.sort_unstable_by_key(|brick| brick.a.z);

        let n = self.bricks.len();

        // let the bricks settle
        for i in 0..n {
            let brick = &self.bricks[i];

            let max_z = self.bricks[..i]
                .iter()
                .filter(|&b| b.overlap(brick))
                .map(|brick| brick.b.z)
                .max()
                .unwrap_or(0)
                + 1;

            let brick = self.bricks.get_mut(i).unwrap();
            let delta = -brick.a.z + max_z;
            brick.b.z += delta;
            brick.a.z += delta;
        }

        // who supports whom ?
        for i in 0..n {
            self.supports.insert(i, HashSet::new());
            self.supported_by.insert(i, HashSet::new());
        }

        for (i, upper) in self.bricks.iter().enumerate() {
            for (j, lower) in self.bricks[..i].iter().enumerate() {
                if upper.overlap(lower) && upper.a.z == lower.b.z + 1 {
                    self.supported_by.get_mut(&i).unwrap().insert(j);
                    self.supports.get_mut(&j).unwrap().insert(i);
                }
            }
        }
    }

    fn part1(&self) -> usize {
        (0..self.bricks.len())
            .filter(|j| {
                self.supports[&j]
                    .iter()
                    .all(|i| self.supported_by[i].len() >= 2)
            })
            .count()
    }

    fn part2(&self) -> usize {
        (0..self.bricks.len())
            .map(|j| {
                let mut q = VecDeque::new();
                let mut fall = HashSet::new();

                for &i in &self.supports[&j] {
                    if self.supported_by[&i].len() == 1 {
                        q.push_back(i);
                        fall.insert(i);
                    }
                }

                while let Some(j) = q.pop_front() {
                    let e = self.supports[&j]
                        // optimization: remove the already fallen bricks
                        .difference(&fall)
                        .copied()
                        .collect::<Vec<_>>();
                    for k in e {
                        if fall.is_superset(&self.supported_by[&k]) {
                            q.push_back(k);
                            fall.insert(k);
                        }
                    }
                }

                fall.len()
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    Some(puzzle.part1())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    Some(puzzle.part2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
