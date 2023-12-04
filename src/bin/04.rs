use std::{
    collections::hash_map::Entry::Occupied,
    collections::{HashMap, HashSet},
    io::Write,
};

use regex::Regex;

advent_of_code::solution!(4);
#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let card_re = Regex::new(r"Card ([0-9 ]+):([0-9 ]+)\|([0-9 ]+)").unwrap();
    let space_re = Regex::new(r"\s+").unwrap();
    let mut cards = vec![];
    let mut card_copies = HashMap::new();
    for line in lines {
        let (_, [id_str, winning_str, numbers_str]) = card_re.captures(line).unwrap().extract();
        let id = id_str.parse::<u32>().unwrap_or(0);
        let mut winning_numbers = HashSet::new();
        let mut numbers = vec![];
        for num_str in space_re.split(winning_str) {
            let num = num_str.parse::<u32>().unwrap_or(0);
            if num != 0 {
                winning_numbers.insert(num);
            }
        }
        for num_str in space_re.split(numbers_str) {
            let num = num_str.parse::<u32>().unwrap_or(0);
            if num != 0 {
                numbers.push(num);
            }
        }
        let card = Card {
            id,
            winning_numbers,
            numbers,
        };
        cards.push(card);
        card_copies.insert(id, 1);
    }
    let mut sum = 0;
    for card in &cards {
        let mut num = 0;
        for number in &card.numbers {
            if card.winning_numbers.contains(number) {
                num += 1;
            }
        }
        let mut score = 0;
        if num >= 1 {
            score = 1;
        }
        for _ in 1..num {
            score *= 2;
        }
        sum += score;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let card_re = Regex::new(r"Card\s+([0-9]+):([0-9 ]+)\|([0-9 ]+)").unwrap();
    let space_re = Regex::new(r"\s+").unwrap();
    let mut cards = vec![];
    let mut card_copies = HashMap::new();
    for line in lines {
        let (_, [id_str, winning_str, numbers_str]) = card_re.captures(line).unwrap().extract();
        let id = id_str.parse::<u32>().unwrap_or(0);
        let mut winning_numbers = HashSet::new();
        let mut numbers = vec![];
        for num_str in space_re.split(winning_str) {
            let num = num_str.parse::<u32>().unwrap_or(0);
            if num != 0 {
                winning_numbers.insert(num);
            }
        }
        for num_str in space_re.split(numbers_str) {
            let num = num_str.parse::<u32>().unwrap_or(0);
            if num != 0 {
                numbers.push(num);
            }
        }
        let card = Card {
            id,
            winning_numbers,
            numbers,
        };
        cards.push(card);
        card_copies.insert(id, 1);
    }
    let mut total_num_cards = 0;
    for card in &cards {
        let mut num = 0;
        for number in &card.numbers {
            if card.winning_numbers.contains(number) {
                num += 1;
            }
        }
        for i in card.id + 1..card.id + num + 1 {
            if card_copies.contains_key(&i) {
                card_copies.insert(i, card_copies[&i] + card_copies[&card.id]);
            }
        }
        total_num_cards += card_copies[&card.id];
    }
    Some(total_num_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
