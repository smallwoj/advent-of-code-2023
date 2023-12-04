use regex::Regex;
use std::{cmp::min, collections::HashMap};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let number_re = Regex::new(r"[0-9]").unwrap();
    let chars = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut number_indices = vec![];
    for (i, line) in chars.iter().enumerate() {
        let mut curr_start = None;
        for j in 0..line.len() {
            if number_re.is_match(chars[i][j].to_string().as_str()) {
                if curr_start.is_none() {
                    curr_start = Some((i, j));
                }
            } else if let Some(start_index) = curr_start {
                number_indices.push((start_index, (i, j)));
                curr_start = None;
            }
        }
        if let Some(start_index) = curr_start {
            number_indices.push((start_index, (i, line.len())));
        }
    }
    let mut sum = 0;
    for (start_indices, end_indices) in number_indices {
        let mut symbol_found = false;
        if start_indices.0 > 0 {
            let sub_index_start = if start_indices.1 != 0 {
                start_indices.1 - 1
            } else {
                0
            };
            for k in sub_index_start..min(end_indices.1 + 1, chars[start_indices.0].len()) {
                if chars[start_indices.0 - 1][k] != '.'
                    && !number_re.is_match(chars[start_indices.0 - 1][k].to_string().as_str())
                {
                    symbol_found = true;
                }
            }
        }
        if start_indices.0 < chars.len() - 1 {
            let sub_index_start = if start_indices.1 != 0 {
                start_indices.1 - 1
            } else {
                0
            };
            for k in sub_index_start..min(end_indices.1 + 1, chars[start_indices.0].len()) {
                if chars[start_indices.0 + 1][k] != '.'
                    && !number_re.is_match(chars[start_indices.0 + 1][k].to_string().as_str())
                {
                    symbol_found = true;
                }
            }
        }
        if start_indices.1 != 0
            && chars[start_indices.0][start_indices.1 - 1] != '.'
            && !number_re.is_match(
                chars[start_indices.0][start_indices.1 - 1]
                    .to_string()
                    .as_str(),
            )
        {
            symbol_found = true;
        }
        if end_indices.1 != chars[end_indices.0].len()
            && chars[end_indices.0][end_indices.1] != '.'
            && !number_re.is_match(chars[end_indices.0][end_indices.1].to_string().as_str())
        {
            symbol_found = true;
        }
        if symbol_found {
            let char_num = &lines[start_indices.0][start_indices.1..end_indices.1];
            let num = char_num.parse::<u32>().unwrap();
            sum += num;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let number_re = Regex::new(r"[0-9]").unwrap();
    let chars = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut number_indices = vec![];
    let mut gears = HashMap::new() as HashMap<(u32, u32), Vec<u32>>;
    for (i, line) in chars.iter().enumerate() {
        let mut curr_start = None;
        for j in 0..line.len() {
            if number_re.is_match(chars[i][j].to_string().as_str()) {
                if curr_start.is_none() {
                    curr_start = Some((i, j));
                }
            } else if let Some(start_index) = curr_start {
                number_indices.push((start_index, (i, j)));
                curr_start = None;
            }
            if chars[i][j] == '*' {
                gears.insert((i as u32, j as u32), vec![]);
            }
        }
        if let Some(start_index) = curr_start {
            number_indices.push((start_index, (i, line.len())));
        }
    }
    let mut sum = 0;
    for (start_indices, end_indices) in number_indices {
        let mut gear_pos = None;
        if start_indices.0 > 0 {
            let sub_index_start = if start_indices.1 != 0 {
                start_indices.1 - 1
            } else {
                0
            };
            for k in sub_index_start..min(end_indices.1 + 1, chars[start_indices.0].len()) {
                if chars[start_indices.0 - 1][k] != '.'
                    && !number_re.is_match(chars[start_indices.0 - 1][k].to_string().as_str())
                {
                    gear_pos = Some((start_indices.0 - 1, k));
                }
            }
        }
        if start_indices.0 < chars.len() - 1 {
            let sub_index_start = if start_indices.1 != 0 {
                start_indices.1 - 1
            } else {
                0
            };
            for k in sub_index_start..min(end_indices.1 + 1, chars[start_indices.0].len()) {
                if chars[start_indices.0 + 1][k] != '.'
                    && !number_re.is_match(chars[start_indices.0 + 1][k].to_string().as_str())
                {
                    gear_pos = Some((start_indices.0 + 1, k));
                }
            }
        }
        if start_indices.1 != 0
            && chars[start_indices.0][start_indices.1 - 1] != '.'
            && !number_re.is_match(
                chars[start_indices.0][start_indices.1 - 1]
                    .to_string()
                    .as_str(),
            )
        {
            gear_pos = Some((start_indices.0, start_indices.1 - 1))
        }
        if end_indices.1 != chars[end_indices.0].len()
            && chars[end_indices.0][end_indices.1] != '.'
            && !number_re.is_match(chars[end_indices.0][end_indices.1].to_string().as_str())
        {
            gear_pos = Some((end_indices.0, end_indices.1));
        }
        if let Some(gear_loc) = gear_pos {
            let char_num = &lines[start_indices.0][start_indices.1..end_indices.1];
            let num = char_num.parse::<u32>().unwrap();
            let converted_gear_loc = (gear_loc.0 as u32, gear_loc.1 as u32);
            if let Some(val) = gears.get_mut(&converted_gear_loc) {
                val.push(num)
            };
        }
    }
    for parts in gears.values() {
        let mut val = 1;
        for part in parts {
            val *= part;
        }
        if parts.len() >= 2 {
            sum += val;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
