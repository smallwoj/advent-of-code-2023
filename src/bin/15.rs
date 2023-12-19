use std::{collections::HashMap, u32};

use regex::Regex;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split(',').fold(0, |value_sum, step_str| {
        let mut curr_val = 0;
        for c in step_str.chars() {
            curr_val += c as u32;
            curr_val *= 17;
            curr_val %= 256;
        }
        value_sum + curr_val
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    fn hash(string: &str) -> u32 {
        let mut curr_val = 0;
        for c in string.chars() {
            curr_val += c as u32;
            curr_val *= 17;
            curr_val %= 256;
        }
        curr_val
    }
    let mut boxmap = HashMap::new();
    let set_operation_re = Regex::new(r"([a-zA-Z]+)=([0-9]+)").unwrap();
    let del_operation_re = Regex::new(r"([a-zA-Z]+)-").unwrap();
    for step_str in input.split(',') {
        if let Some(result) = set_operation_re.captures(step_str) {
            let (_, [label, num_str]) = result.extract();
            let num = num_str.parse::<u32>().unwrap_or(0);
            let key = hash(label);
            let boxes = boxmap.entry(key).or_insert(vec![]);
            if let Some((idx, _)) = boxes.iter().enumerate().find(|(_, (l, _))| l == &label) {
                boxes[idx] = (label, num);
            } else {
                boxes.push((label, num));
            }
        } else if let Some(result) = del_operation_re.captures(step_str) {
            let (_, [label]) = result.extract();
            let key = hash(label);
            let boxes = boxmap.entry(key).or_insert(vec![]);
            let removed = boxes
                .iter()
                .filter(|(l, _)| l != &label)
                .map(|x| x.to_owned())
                .collect::<Vec<(&str, u32)>>();
            boxmap.insert(key, removed);
        } else {
            panic!("Could not parse step {}", step_str);
        }
    }
    Some(
        boxmap
            .keys()
            .map(|key| {
                let boxes = &boxmap[key];
                boxes
                    .iter()
                    .enumerate()
                    .map(|(slot, (_, focal_length))| (key + 1) * (slot as u32 + 1) * focal_length)
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
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
