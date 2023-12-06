advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let times = &lines[0]
        .split_whitespace()
        .map(|tok| tok.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>()[1..];
    let records = &lines[1]
        .split_whitespace()
        .map(|tok| tok.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>()[1..];
    let mut prod = 1;
    for (time, record) in times.iter().zip(records.iter()) {
        let mut n = 0;
        for i in 0..=(*time as i32) {
            let dist = -i * (i - *time as i32);
            if dist > *record as i32 {
                n += 1;
            }
        }
        prod *= n;
    }
    Some(prod)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let time = &lines[0]
        .split_whitespace()
        .map(|tok| tok.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>()
        .iter()
        .fold("".to_string(), |acc, e| format!("{}{}", acc, e))
        .parse::<i64>()
        .unwrap_or(0);
    let record = &lines[1]
        .split_whitespace()
        .map(|tok| tok.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>()
        .iter()
        .fold("".to_string(), |acc, e| format!("{}{}", acc, e))
        .parse::<i64>()
        .unwrap_or(0);
    let mut n = 0;
    for i in 0..=*time {
        let dist = -i * (i - time);
        if dist > *record {
            n += 1;
        }
    }
    Some(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
