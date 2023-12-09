use std::cmp::Ordering;
advent_of_code::solution!(7);

#[derive(Debug)]
struct CamelCard<'a> {
    hand: &'a str,
    bid: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<CamelCard> = vec![];
    for line in input.lines() {
        let hand = &line[..5];
        let bid = line[6..].parse().unwrap();
        hands.push(CamelCard { hand, bid });
    }
    hands.sort_by(|a, b| {
        score_hand(a.hand)
            // sort by score first
            .cmp(&score_hand(b.hand))
            // then by card left-to-right
            .then_with(|| {
                a.hand.chars().zip(b.hand.chars())
                    .find_map(|(char_a, char_b)| {
                        let cmp = card_to_num(char_a).cmp(&card_to_num(char_b));
                        if cmp == Ordering::Equal { None } else { Some(cmp) }
                    })
                    .unwrap_or(Ordering::Equal)
            })
    });
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum(),
    )
}

fn card_to_num(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn score_hand(hand: &str) -> u32 {
    let mut freqs = [0u32; 15];
    for c in hand.chars() {
        freqs[card_to_num(c) as usize] += 1;
    }

    freqs.sort();
    match freqs[freqs.len() - 1] {
        5 => 6, // five of a kind
        4 => 5, // four of a kind
        3 => match freqs[freqs.len() - 2] {
            2 => 4, // full house
            _ => 3, // three of a kind
        },
        2 => match freqs[freqs.len() - 2] {
            2 => 2, // two pair
            _ => 1, // one pair
        },
        1 => 0, // one pair
        _ => unreachable!(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<CamelCard> = vec![];
    for line in input.lines() {
        let hand = &line[..5];
        let bid = line[6..].parse().unwrap();
        hands.push(CamelCard { hand, bid });
    }
    hands.sort_by(|a, b| {
        score_hand_2(a.hand)
            // sort by score first
            .cmp(&score_hand_2(b.hand))
            // then by card left-to-right
            .then_with(|| {
                a.hand.chars().zip(b.hand.chars())
                    .find_map(|(char_a, char_b)| {
                        let cmp = card_to_num_2(char_a).cmp(&card_to_num_2(char_b));
                        if cmp == Ordering::Equal { None } else { Some(cmp) }
                    })
                    .unwrap_or(Ordering::Equal)
            })
    });
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum(),
    )
}

fn card_to_num_2(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 1, // weakest
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn score_hand_2(hand: &str) -> u32 {
    let mut freqs = [0u32; 15];
    let mut num_jokers = 0;
    for c in hand.chars() {
        if c == 'J' {
            num_jokers += 1;
            continue
        }
        freqs[card_to_num(c) as usize] += 1;
    }

    freqs.sort();
    freqs[freqs.len() - 1] += num_jokers;
    match freqs[freqs.len() - 1] {
        5 => 6, // five of a kind
        4 => 5, // four of a kind
        3 => match freqs[freqs.len() - 2] {
            2 => 4, // full house
            _ => 3, // three of a kind
        },
        2 => match freqs[freqs.len() - 2] {
            2 => 2, // two pair
            _ => 1, // one pair
        },
        1 => 0, // one pair
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
