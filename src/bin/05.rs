advent_of_code::solution!(5);

#[derive(Debug)]
struct Entry {
    delta: isize,
    src_start: isize,
    range: isize,
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut lines = input.lines();

    let seeds = get_nums(lines.next().unwrap());
    let maps = generate_maps(input);

    seeds.into_iter().map(|seed| resolve(seed, &maps)).min()
}

fn generate_maps(input: &str) -> Vec<Vec<Entry>> {
    // skip the seeds
    let lines = input.lines().skip(1);
    let mut maps: Vec<Vec<Entry>> = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }
        // start a new map
        if line.contains("map:") {
            maps.push(vec![]);
            continue;
        }

        let almanac = get_nums(line);
        let entry = Entry {
            delta: almanac[0] - almanac[1],
            src_start: almanac[1],
            range: almanac[2],
        };
        let last = maps.len() - 1;
        maps[last].push(entry);
    }
    maps
}

fn resolve(mut seed: isize, maps: &Vec<Vec<Entry>>) -> isize {
    for map in maps {
        for entry in map {
            if entry.src_start <= seed && seed < entry.src_start + entry.range {
                seed += entry.delta;
                break;
            }
        }
    }
    seed
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut lines = input.lines();

    let seed_range = get_nums(lines.next().unwrap());
    let mut seeds = vec![];
    let mut i = 0;
    while i + 1 < seed_range.len() {
        for j in 0..seed_range[i + 1] {
            seeds.push(seed_range[i] + j);
        }
        i += 2;
    }
    let maps = generate_maps(input);

    seeds.into_iter().map(|seed| resolve(seed, &maps)).min()
}

fn get_nums(s: &str) -> Vec<isize> {
    let mut nums = vec![];
    let mut num = 0;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap() as isize;
            num_found = true;
        } else if num_found {
            nums.push(num);
            num = 0;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num);
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
