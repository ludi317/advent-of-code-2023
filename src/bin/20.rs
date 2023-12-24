use std::collections::VecDeque;
use hashbrown::HashMap;
use num::integer::lcm;
advent_of_code::solution!(20);

#[derive(Debug)]
pub struct Input {
    modules: HashMap<String, Module>,
    broadcast_target: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction, // unnecessary
}

#[derive(Debug, Clone, PartialEq)]
enum Memory {
    On,
    Off,
    Map(HashMap<String, Pulse>),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pulse {
    Low,
    High
}

impl From<char> for ModuleType {
    fn from(c: char) -> Self {
        match c {
            '%' => Self::FlipFlop,
            '&' => Self::Conjunction,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
    memory: Memory,
}

impl Module {
    fn new(name: String, module_type: ModuleType, outputs: Vec<String>) -> Self {
        let memory = match module_type {
            ModuleType::FlipFlop => Memory::Off,
            ModuleType::Conjunction => Memory::Map(HashMap::new()),
        };

        Self {
            name,
            module_type,
            outputs,
            memory,
        }
    }
}

pub fn input_generator(input: &str) -> Input {
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut broadcast_target = vec![];

    for line in input.lines() {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        if module == "broadcaster" {
            broadcast_target = outputs;
        } else {
            let module_type = module.chars().next().unwrap();
            let name = module[1..].to_string();

            modules.insert(name.clone(), Module::new(name, module_type.into(), outputs));
        }
    }

    // Setup initial memory
    for (name, module) in modules.clone().iter() {
        for output in module.outputs.iter() {
            if let Some(m) = modules.get_mut(output) {
                if let ModuleType::Conjunction = m.module_type {
                    if let Memory::Map(ref mut map) = m.memory {
                        map.insert(name.clone(), Pulse::Low);
                    }
                }
            }
        }
    }

    Input {
        modules,
        broadcast_target,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input_generator(input);
    let mut modules = input.modules;
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        // A low pulse from the button press is sent each time
        low_pulses += 1;
        let mut queue = VecDeque::new();

        for target in input.broadcast_target.iter() {
            queue.push_back(("broadcaster".to_string(), target.clone(), Pulse::Low));
        }

        while let Some((from, to, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            };

            // Handles unknown modules
            if !modules.contains_key(&to) {
                continue;
            };

            let module = modules.get_mut(&to).unwrap();

            match module.module_type {
                ModuleType::FlipFlop => {
                    if pulse == Pulse::Low {
                        module.memory = if module.memory == Memory::Off {
                            Memory::On
                        } else {
                            Memory::Off
                        };
                        let next_pulse = if module.memory == Memory::On {
                            Pulse::High
                        } else {
                            Pulse::Low
                        };

                        for output in module.outputs.iter() {
                            queue.push_back(
                                (module.name.clone(), output.clone(), next_pulse)
                            );
                        }
                    }
                }

                ModuleType::Conjunction => {
                    if let Memory::Map(ref mut map) = module.memory {
                        map.insert(from, pulse);

                        let next_pulse = if map.values().all(|x| *x == Pulse::High) {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };

                        for output in module.outputs.iter() {
                            queue.push_back(
                                (module.name.clone(), output.clone(), next_pulse)
                            );
                        }
                    }
                }
            }
        }
    }

    Some(low_pulses * high_pulses)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input_generator(input);
    let mut modules = input.modules.clone();
    let mut button_presses = 0;

    // Find which module feeds into "rx". This should only be one.
    // This is "zh" in my input.
    let feed = input
        .modules
        .values()
        .find(|module| module.outputs.contains(&"rx".to_string()))
        .unwrap();

    // Find the modules that feed into the module, "zh", that feeds into "rx".
    // We'll use these to find the cycle length and then we can calculate the
    // LCM of the cycle lengths.
    let mut cycle_lengths: HashMap<String, u64> = HashMap::new();

    let mut seen = input
        .modules
        .values()
        .filter(|module| module.outputs.contains(&feed.name))
        .map(|module| (module.name.clone(), 0))
        .collect::<HashMap<String, u64>>();

    let fewest_button_presses =
        'outer: loop {
            button_presses += 1;

            let mut queue = VecDeque::new();

            for target in input.broadcast_target.iter() {
                queue.push_back(("broadcaster".to_string(), target.clone(), Pulse::Low));
            }

            while let Some((from, to, pulse)) = queue.pop_front() {
                // Handles unknown modules
                if !modules.contains_key(&to) {
                    continue;
                };

                let module = modules.get_mut(&to).unwrap();

                // We only care about the module that feeds into "zh"
                // and we only care about high pulses
                if module.name == feed.name && pulse == Pulse::High {
                    seen.entry(from.clone()).and_modify(|x| *x += 1);

                    // Update the cycle length
                    if !cycle_lengths.contains_key(&from) {
                        cycle_lengths.insert(from.clone(), button_presses);
                    }

                    // We've seen all the modules that feed into "zh"
                    // Calculate the LCM of the cycle lengths and break
                    if seen.values().all(|x| *x == 1) {
                        break 'outer cycle_lengths
                            .values()
                            .fold(1, |acc, x| lcm(acc, *x as i64));
                    }
                }

                match module.module_type {
                    ModuleType::FlipFlop => {
                        if pulse == Pulse::Low {
                            module.memory = if module.memory == Memory::Off {
                                Memory::On
                            } else {
                                Memory::Off
                            };
                            let next_pulse = if module.memory == Memory::On {
                                Pulse::High
                            } else {
                                Pulse::Low
                            };

                            for output in module.outputs.iter() {
                                queue.push_back((
                                    module.name.clone(),
                                    output.clone(),
                                    next_pulse,
                                ));
                            }
                        }
                    }

                    ModuleType::Conjunction => {
                        if let Memory::Map(ref mut map) = module.memory {
                            map.insert(from, pulse);

                            let next_pulse = if map.values().all(|x| *x == Pulse::High) {
                                Pulse::Low
                            } else {
                                Pulse::High
                            };

                            for output in module.outputs.iter() {
                                queue.push_back((
                                    module.name.clone(),
                                    output.clone(),
                                    next_pulse,
                                ));
                            }
                        }
                    }
                }
            }
        };

    Some(fewest_button_presses as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

}
