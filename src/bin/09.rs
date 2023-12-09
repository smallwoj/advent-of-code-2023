advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut histories = vec![];
    for line in lines {
        histories.push(
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap_or(0))
                .collect::<Vec<i64>>(),
        );
    }
    let mut values = 0;
    fn extrapolate(sequence: Vec<i64>) -> i64 {
        if sequence.iter().all(|&el| el == 0) {
            0
        } else {
            let mut diffs = vec![];
            for i in 1..sequence.len() {
                let diff = sequence[i] - sequence[i - 1];
                diffs.push(diff);
            }
            sequence.last().unwrap() + extrapolate(diffs)
        }
    }
    for history in histories {
        values += extrapolate(history);
    }
    Some(values)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut histories = vec![];
    for line in lines {
        histories.push(
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap_or(0))
                .rev()
                .collect::<Vec<i64>>(),
        );
    }
    let mut values = 0;
    fn extrapolate(sequence: Vec<i64>) -> i64 {
        if sequence.iter().all(|&el| el == 0) {
            0
        } else {
            let mut diffs = vec![];
            for i in 1..sequence.len() {
                let diff = sequence[i] - sequence[i - 1];
                diffs.push(diff);
            }
            sequence.last().unwrap() + extrapolate(diffs)
        }
    }
    for history in histories {
        values += extrapolate(history);
    }
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
