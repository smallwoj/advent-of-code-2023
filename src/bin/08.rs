use std::{collections::HashMap, io::Write};

use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let [sequence, node_defs_str] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("could not parse line {}", input);
    };
    let mut sequence_iter = sequence.chars().cycle();
    let mut node_defs = HashMap::new();
    let node_def_re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    let lines = node_defs_str.split('\n').collect::<Vec<&str>>();
    for line in lines {
        let (_, [node, left, right]) = node_def_re.captures(line).unwrap().extract();
        node_defs.insert(node, (left, right));
    }
    let mut found = false;
    let mut steps = 0;
    let mut curr_node = "AAA";
    let mut curr_seq = sequence_iter.next().unwrap();
    while !found {
        if curr_node == "ZZZ" {
            found = true;
        } else {
            steps += 1;
            curr_node = match curr_seq {
                'L' => node_defs[curr_node].0,
                'R' => node_defs[curr_node].1,
                _ => "",
            };
            curr_seq = sequence_iter.next().unwrap();
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let [sequence, node_defs_str] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("could not parse line {}", input);
    };
    let mut node_defs = HashMap::new();
    let node_def_re = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
    let lines = node_defs_str.split('\n').collect::<Vec<&str>>();
    for line in lines {
        let (_, [node, left, right]) = node_def_re.captures(line).unwrap().extract();
        node_defs.insert(node, (left, right));
    }
    let curr_nodes = node_defs
        .keys()
        .copied()
        .filter(|&node| node.ends_with('A'))
        .collect::<Vec<&str>>();
    let mut all_steps = vec![];
    for start in curr_nodes {
        let mut curr_node = start;
        let mut found = false;
        let mut steps = 0;
        let mut sequence_iter = sequence.chars().cycle();
        let mut curr_seq = sequence_iter.next().unwrap();
        while !found {
            std::io::stdout().flush().unwrap();
            if curr_node.ends_with('Z') {
                found = true;
            } else {
                steps += 1;
                curr_node = match curr_seq {
                    'L' => node_defs[curr_node].0,
                    'R' => node_defs[curr_node].1,
                    _ => "",
                };
                curr_seq = sequence_iter.next().unwrap();
            }
        }
        all_steps.push(steps);
    }
    fn gcd(x: u64, y: u64) -> u64 {
        let mut max = x;
        let mut min = y;
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
        }
    }
    fn lcm(x: u64, y: u64) -> u64 {
        x * y / gcd(x, y)
    }
    let mut acc = all_steps[0];
    for x in &mut all_steps[1..] {
        acc = lcm(acc, *x);
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
        let second_example = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        result = part_one(second_example);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let mut result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
        let second_example = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        result = part_two(second_example);
        assert_eq!(result, Some(6));
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        result = part_two(input);
        assert_eq!(result, Some(6));
    }
}
