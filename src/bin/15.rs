use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .split(',')
        .map(|s| s.bytes().fold(0, |acc, b| (17 * (acc + b as u32)) % 256))
        .sum::<u32>();

    Some(sum)
}

#[derive(Debug)]
struct Lens {
    f: usize,
    pos: usize,
}

pub fn part_two(input: &str) -> Option<usize> {
    // init 2 data structures:
    // 1. vec of hashmap name -> {focal length, pos} per box
    // 2. vec of highest position per box
    let num_boxes = 256;
    let mut boxes: Vec<HashMap<&str, Lens>> = Vec::with_capacity(num_boxes);
    let mut highest_pos = vec![0usize; num_boxes];
    for _ in 0..num_boxes {
        boxes.push(HashMap::new());
    }

    for s in input.split(',') {
        let bs = s.as_bytes();
        let dash_idx = s.find('-');
        let idx = dash_idx.unwrap_or_else(|| s.find('=').unwrap());
        let name = &s[..idx];
        let box_num = bs[..idx]
            .iter()
            .fold(0, |acc, &b| (17 * (acc + b as usize)) % 256);

        if dash_idx.is_some() {
            // remove lens
            boxes[box_num].remove(name);
        } else {
            // focal length present
            let f = (bs[idx + 1] - b'0') as usize;
            let lens_entry = boxes[box_num].entry(&name);
            match lens_entry {
                Entry::Occupied(mut entry) => {
                    // If the entry exists, update f and keep pos
                    let lens = entry.get_mut();
                    lens.f = f;
                }
                Entry::Vacant(entry) => {
                    // If the entry does not exist, insert a new one with a new highest pos
                    entry.insert(Lens {
                        f,
                        pos: highest_pos[box_num],
                    });
                    highest_pos[box_num] += 1;
                }
            }
        }
    }

    // sort values by pos
    let mut ans = 0;
    for (i, box_map) in boxes.iter().enumerate() {
        let mut lenses: Vec<_> = box_map.values().collect();
        lenses.sort_by_key(|&lens| lens.pos);
        for (j, p) in lenses.iter().enumerate() {
            ans += (i + 1) * (j + 1) * p.f;
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
