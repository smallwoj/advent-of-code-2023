use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    /// true if hand 1 is ranked higher
    fn second_ordering(hand1: &Hand, hand2: &Hand) -> Ordering {
        let ordering = "AKQJT98765432";
        for (c1, c2) in hand1.hand.chars().zip(hand2.hand.chars()) {
            let c1_pos = ordering.chars().position(|el| el == c1);
            let c2_pos = ordering.chars().position(|el| el == c2);
            match c1_pos.cmp(&c2_pos) {
                Ordering::Greater => return Ordering::Less,
                Ordering::Less => return Ordering::Greater,
                _ => {}
            }
        }
        Ordering::Equal
    }
    #[derive(Eq)]
    struct Hand {
        hand: String,
        bet: u32,
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let mut freq_counts_self = HashMap::new();
            let mut freq_counts_other = HashMap::new();
            for c in self.hand.chars() {
                *freq_counts_self.entry(c).or_insert(0) += 1;
            }
            for c in other.hand.chars() {
                *freq_counts_other.entry(c).or_insert(0) += 1;
            }
            let mut self_counts: Vec<_> = freq_counts_self.values().collect();
            self_counts.sort();
            self_counts.reverse();
            let self_type = match self_counts[..] {
                [5] => 7,
                [4, 1] => 6,
                [3, 2] => 5,
                [3, 1, 1] => 4,
                [2, 2, 1] => 3,
                [2, 1, 1, 1] => 2,
                [1, 1, 1, 1, 1] => 1,
                _ => 0,
            };
            let mut other_counts: Vec<_> = freq_counts_other.values().collect();
            other_counts.sort();
            other_counts.reverse();
            let other_type = match other_counts[..] {
                [5] => 7,
                [4, 1] => 6,
                [3, 2] => 5,
                [3, 1, 1] => 4,
                [2, 2, 1] => 3,
                [2, 1, 1, 1] => 2,
                [1, 1, 1, 1, 1] => 1,
                _ => 0,
            };
            match self_type.cmp(&other_type) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => second_ordering(self, other),
            }
        }
    }
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.hand == other.hand
        }
    }
    let lines = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .collect::<Vec<&str>>();
    let mut hands = vec![];
    for line in lines {
        let [hand_str, bet_str] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("could not parse line {}", line);
        };
        let hand = String::from(hand_str);
        let bet = bet_str.parse::<u32>().unwrap_or(0);
        let hand = Hand { hand, bet };
        hands.push(hand);
    }
    hands.sort();
    let mut winnings = 0;
    for (idx, hand) in hands.iter().enumerate() {
        winnings += hand.bet * (idx as u32 + 1);
    }
    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    /// true if hand 1 is ranked higher
    fn second_ordering(hand1: &Hand, hand2: &Hand) -> Ordering {
        let ordering = "AKQT98765432J";
        for (c1, c2) in hand1.hand.chars().zip(hand2.hand.chars()) {
            let c1_pos = ordering.chars().position(|el| el == c1);
            let c2_pos = ordering.chars().position(|el| el == c2);
            match c1_pos.cmp(&c2_pos) {
                Ordering::Greater => return Ordering::Less,
                Ordering::Less => return Ordering::Greater,
                _ => {}
            }
        }
        Ordering::Equal
    }
    #[derive(Eq)]
    struct Hand {
        hand: String,
        bet: u32,
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            fn get_type(hand: &str) -> u32 {
                let mut freq_counts = HashMap::new();
                for c in hand.chars() {
                    *freq_counts.entry(c).or_insert(0) += 1;
                }
                let mut counts: Vec<_> = freq_counts.values().collect();
                counts.sort();
                counts.reverse();
                match counts[..] {
                    [5] => 7,
                    [4, 1] => 6,
                    [3, 2] => 5,
                    [3, 1, 1] => 4,
                    [2, 2, 1] => 3,
                    [2, 1, 1, 1] => 2,
                    [1, 1, 1, 1, 1] => 1,
                    _ => 0,
                }
            }
            fn generate_top_hand_type(hand: &str) -> u32 {
                let other_chars = "AKQT98765432";
                let mut potential_hands = vec![String::from("")];
                for c in hand.chars() {
                    let mut new_potential_hands = vec![];
                    for hand in potential_hands.clone() {
                        if c != 'J' {
                            let new_hand = format!("{}{}", hand, c);
                            new_potential_hands.push(new_hand);
                        } else {
                            for c2 in other_chars.chars() {
                                let new_hand = format!("{}{}", hand, c2);
                                new_potential_hands.push(new_hand);
                            }
                        }
                    }
                    potential_hands.clone_from(&new_potential_hands)
                }
                potential_hands
                    .iter()
                    .map(|hand| get_type(hand))
                    .max()
                    .unwrap_or(0)
            }
            let self_type = generate_top_hand_type(&self.hand);
            let other_type = generate_top_hand_type(&other.hand);
            match self_type.cmp(&other_type) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => second_ordering(self, other),
            }
        }
    }
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.hand == other.hand
        }
    }
    let lines = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .collect::<Vec<&str>>();
    let mut hands = vec![];
    for line in lines {
        let [hand_str, bet_str] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("could not parse line {}", line);
        };
        let hand = String::from(hand_str);
        let bet = bet_str.parse::<u32>().unwrap_or(0);
        let hand = Hand { hand, bet };
        hands.push(hand);
    }
    hands.sort();
    let mut winnings = 0;
    for (idx, hand) in hands.iter().enumerate() {
        winnings += hand.bet * (idx as u32 + 1);
    }
    Some(winnings)
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
