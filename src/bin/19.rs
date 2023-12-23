use advent_of_code::get_nums_usize;
use hashbrown::HashMap;
advent_of_code::solution!(19);

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Copy, Clone)]
struct PartRange<'a> {
    x: [usize; 2],
    m: [usize; 2],
    a: [usize; 2],
    s: [usize; 2],
    starting_key: &'a str,
}

impl PartRange<'_> {
    fn count(&self) -> usize {
        (self.x[1] - self.x[0] + 1)
            * (self.m[1] - self.m[0] + 1)
            * (self.a[1] - self.a[0] + 1)
            * (self.s[1] - self.s[0] + 1)
    }
    fn get_field_by_name(&self, field: &str) -> [usize; 2] {
        match field {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!(),
        }
    }
    fn set_field_by_name(&mut self, field: &str, val: [usize; 2]) {
        match field {
            "x" => self.x = val,
            "m" => self.m = val,
            "a" => self.a = val,
            "s" => self.s = val,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut part_found = false;
    let mut parts: Vec<Part> = vec![];
    let mut workflow: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            part_found = true;
            continue;
        }
        if !part_found {
            let curly = line.find('{').unwrap();
            let rest = &line[curly + 1..line.len() - 1];
            let rules = rest.split(',').collect();
            workflow.insert(&line[..curly], rules);
        } else {
            let nums = get_nums_usize(line);
            parts.push(Part {
                x: nums[0],
                m: nums[1],
                a: nums[2],
                s: nums[3],
            })
        }
    }

    let ans = parts
        .iter()
        .map(|part| {
            let mut key = "in";
            while key != "A" && key != "R" {
                let rules = &workflow[&key];
                for rule in rules {
                    let colon = rule.find(':');
                    if colon.is_none() {
                        key = rule;
                        break;
                    }
                    let cond = &rule[..colon.unwrap()];
                    let left = match &cond[0..1] {
                        "x" => part.x,
                        "m" => part.m,
                        "a" => part.a,
                        "s" => part.s,
                        _ => unreachable!(),
                    };
                    let comparator = &cond[1..2];
                    let right = (&cond[2..]).parse().unwrap();
                    let res = match comparator {
                        "<" => left < right,
                        _ => left > right,
                    };
                    if res {
                        key = &rule[colon.unwrap() + 1..];
                        break;
                    }
                }
            }
            if key == "R" {
                return 0;
            }
            part.x + part.m + part.a + part.s
        })
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut workflow: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let curly = line.find('{').unwrap();
        let rest = &line[curly + 1..line.len() - 1];
        let rules = rest.split(',').collect();
        workflow.insert(&line[..curly], rules);
    }

    let mut parts: Vec<PartRange> = vec![PartRange {
        x: [1, 4000],
        m: [1, 4000],
        a: [1, 4000],
        s: [1, 4000],
        starting_key: "in",
    }];

    let mut i = 0;
    let mut ans = 0;
    while i < parts.len() {
        let mut key = parts[i].starting_key;
        while key != "A" && key != "R" {
            let rules = &workflow[&key];
            for rule in rules {
                let colon = rule.find(':');

                if colon.is_none() {
                    // default rule matched
                    key = rule;
                    break;
                }
                let cond = &rule[..colon.unwrap()];
                let var_name = &cond[0..1];
                let left_interval = parts[i].get_field_by_name(var_name);
                let comparator = &cond[1..2];
                let right = (&cond[2..]).parse().unwrap();

                let res = if comparator == "<" {
                    // left < right ?
                    if left_interval[1] < right {
                        true
                    } else if left_interval[0] >= right {
                        false
                    } else {
                        // split
                        let mut new_part = parts[i].clone();
                        new_part.set_field_by_name(var_name, [right, left_interval[1]]);
                        new_part.starting_key = key;
                        parts.push(new_part);

                        parts[i].set_field_by_name(var_name, [left_interval[0], right - 1]);
                        true
                    }
                } else {
                    // left > right ?
                    if left_interval[0] > right {
                        true
                    } else if left_interval[1] <= right {
                        false
                    } else {
                        // split
                        let mut new_part = parts[i].clone();
                        new_part.set_field_by_name(var_name, [left_interval[0], right]);
                        new_part.starting_key = key;
                        parts.push(new_part);

                        parts[i].set_field_by_name(var_name, [right + 1, left_interval[1]]);
                        true
                    }
                };
                if res {
                    // rule matched
                    key = &rule[colon.unwrap() + 1..];
                    break;
                }
            }
        }
        if key == "A" {
            ans += parts[i].count()
        }
        i += 1;
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
