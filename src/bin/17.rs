use hashbrown::HashMap;
use pheap::PairingHeap;
use std::usize;
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let graph: Vec<Vec<usize>> = vec![
        vec![0, 4, 0, 0, 0, 0, 0, 8, 0],
        vec![4, 0, 8, 0, 0, 0, 0, 11, 0],
        vec![0, 8, 0, 7, 0, 4, 0, 0, 2],
        vec![0, 0, 7, 0, 9, 14, 0, 0, 0],
        vec![0, 0, 0, 9, 0, 10, 0, 0, 0],
        vec![0, 0, 4, 14, 10, 0, 2, 0, 0],
        vec![0, 0, 0, 0, 0, 2, 0, 1, 6],
        vec![8, 11, 0, 0, 0, 0, 1, 0, 7],
        vec![0, 0, 2, 0, 0, 0, 6, 7, 0],
    ];

    let mut dists: HashMap<usize, usize> = HashMap::new();
    let mut frontier: PairingHeap<usize, usize> = PairingHeap::new();
    frontier.insert(0, 0);
    dists.insert(0, 0);

    while let Some((main_n, _)) = frontier.delete_min() {
        for (n, &d) in graph[main_n].iter().enumerate() {
            if d == 0 {continue}
            if !dists.contains_key(&n) || dists[&n] > dists[&main_n] + d {
                frontier.insert(n, dists[&main_n] + d);
                dists.insert(n, dists[&main_n] + d);
            }
        }
    }



    let mut items: Vec<_> = dists.iter().collect();
    items.sort_by_key(|&(_, v)| v);
    println!("{:?}", items);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
