use hashbrown::HashMap;
use pheap::PairingHeap;
use std::usize;
advent_of_code::solution!(17);

// const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
// const LEFT: usize = 3;

const DIRS: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

// each state in the graph
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: [isize; 2],
    dir: usize,
    streak: usize,
}

pub fn part_one(input: &str) -> Option<isize> {
    let grid: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();

    dijkstra(&grid, 1, 3)
}

fn neighbors(o: &State, min_streak: usize, max_streak: usize) -> Vec<State> {
    let mut neighbors: Vec<State> = vec![];
    let new_pos = |dir: usize| [o.pos[0] + DIRS[dir][0], o.pos[1] + DIRS[dir][1]];
    // straight
    if o.streak < max_streak {
        neighbors.push(State {
            pos: new_pos(o.dir),
            dir: o.dir,
            streak: o.streak + 1,
        })
    }
    if o.streak >= min_streak {
        // left
        let left = (o.dir + 3) % 4;
        neighbors.push(State {
            pos: new_pos(left),
            dir: left,
            streak: 1,
        });
        // right
        let right = (o.dir + 1) % 4;
        neighbors.push(State {
            pos: new_pos(right),
            dir: right,
            streak: 1,
        })
    }
    neighbors
}

fn dijkstra(grid: &Vec<Vec<isize>>, min_streak: usize, max_streak: usize) -> Option<isize> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut frontier: PairingHeap<State, isize> = PairingHeap::new();
    let mut dists: HashMap<State, isize> = HashMap::new();
    for start in [
        State {
            pos: [0, 0],
            dir: RIGHT,
            streak: 1,
        },
        State {
            pos: [0, 0],
            dir: DOWN,
            streak: 1,
        },
    ] {
        frontier.insert(start, 0);
        dists.insert(start, 0);
    }

    while let Some((orig_state, cost)) = frontier.delete_min() {
        if orig_state.pos == [rows - 1, cols - 1] && orig_state.streak >= min_streak {
            return Some(cost);
        }

        for n in neighbors(&orig_state, min_streak, max_streak) {
            if n.pos[0] < 0 || n.pos[0] >= rows || n.pos[1] < 0 || n.pos[1] >= cols {
                continue;
            }
            let next_cost = cost + grid[n.pos[0] as usize][n.pos[1] as usize];
            if !dists.contains_key(&n) || dists[&n] > next_cost {
                dists.insert(n, next_cost);
                frontier.insert(n, next_cost);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<isize> {
    let grid: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();

    dijkstra(&grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
