use hashbrown::{HashMap, HashSet};
advent_of_code::solution!(23);

const DIRS: [[isize; 2]; 4]  = [[0,-1],[0,1], [-1,0],[1,0]];

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<[isize; 2]> = HashSet::new();
    Some(dfs(&grid, &mut visited, [0,1], 0))
}


fn dfs(grid: &Vec<Vec<char>>, visited: &mut HashSet<[isize; 2]>, pos: [isize; 2], path_len: usize) -> usize {
    if pos[0] == (grid.len() - 1) as isize {
        return path_len
    }
    if !visited.insert(pos) {return 0;}
    let mut ans = 0;
    match grid[pos[0] as usize][pos[1] as usize] {
        '>' => ans = ans.max(dfs(grid, visited, [pos[0], pos[1]+1], path_len+1)),
        'v' => ans = ans.max(dfs(grid, visited, [pos[0]+1, pos[1]], path_len+1)),
        '<' => ans = ans.max(dfs(grid, visited, [pos[0], pos[1]-1], path_len+1)),
        '^' => ans = ans.max(dfs(grid, visited, [pos[0]-1, pos[1]], path_len+1)),
        '.' => {
            for dir in DIRS {
                let nr = pos[0] + dir[0];
                let nc = pos[1] + dir[1];
                if nr == -1 || nc == -1 || nr == grid.len() as isize || nc == grid[0].len() as isize || grid[nr as usize][nc as usize] == '#' {
                    continue
                }
                ans = ans.max(dfs(grid, visited, [nr, nc], path_len +1 ))
            }
        }
        _ => unreachable!()
    }

    visited.remove(&pos);
    ans
}


pub fn part_two(input: &str) -> Option<isize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // == Build graph ==
    // graph is pos -> set of its neighbors with (pos, dist)
    let mut graph: HashMap<[isize; 2], HashSet<([isize; 2], isize)>> = HashMap::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '#' {
                continue;
            }

            for dir in DIRS {
                let nr = r as isize + dir[0];
                let nc = c as isize + dir[1];
                if nr == -1 || nc == -1 || nr == grid.len() as isize|| nc == grid[0].len() as isize || grid[nr as usize][nc as usize] == '#' {
                    continue
                }
                graph.entry([r as isize, c as isize]).or_insert_with(HashSet::new).insert(([nr,nc], 1));
            }
        }
    }

    // == Collapse graph ==
    let mut dirty = true;
    while dirty {
        dirty = false;
        for key in graph.keys().copied().collect::<Vec<_>>() {
            if graph[&key].len() != 2 {
                continue;
            }

            // only 2 neighbors, kick it out of the graph and connect a to b
            let ((a, a_score), (b, b_score)) = {
                let mut iter = graph[&key].iter();
                (*iter.next().unwrap(), *iter.next().unwrap())
            };

            let a_graph = graph.get_mut(&a).unwrap();
            a_graph.retain(|(pos, _)| *pos != key);
            a_graph.insert((b, a_score + b_score));

            let b_graph = graph.get_mut(&b).unwrap();
            b_graph.retain(|(pos, _)| *pos != key);
            b_graph.insert((a, a_score + b_score));

            graph.remove(&key);
            dirty = true;
        }
    }

    // == Find longest path ==
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut max = 0;

    // while loop which does dfs
    // mimics call stack with "stack"
    stack.push(([0isize,1isize], Some(0)));
    while let Some((pos, dist)) = stack.pop() {
        let Some(distance) = dist else {
            // unvisit
            visited.remove(&pos);
            continue;
        };

        if pos[0] == (grid.len() - 1) as isize {
            max = max.max(distance);
            continue;
        }

        // visit
        if !visited.insert(pos) {
            continue
        }

        // unvisit after all descendants are visited
        stack.push((pos, None));
        for (pos, dist) in &graph[&pos] {
            stack.push((*pos, Some(dist + distance)));
        }
    }

    Some(max)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
