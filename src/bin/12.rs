advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut conditions = vec![];
    for line in lines {
        let [springs, sequence_str] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("could not parse line")
        };
        let mut sequence = vec![];
        for num_str in sequence_str.split(',') {
            sequence.push(num_str.parse::<u32>().unwrap_or(0));
        }
        conditions.push((springs, sequence));
    }

    fn generate_arrangements(original: &str) -> Vec<String> {
        let mut arrangements = vec![String::from("")];
        for c in original.chars() {
            let mut new_arrangements = vec![];
            match c {
                '.' => {
                    for arr in arrangements {
                        new_arrangements.push(format!("{}{}", arr, c));
                    }
                }
                '#' => {
                    for arr in arrangements {
                        new_arrangements.push(format!("{}{}", arr, c));
                    }
                }
                '?' => {
                    for arr in arrangements {
                        new_arrangements.push(format!("{}{}", arr, '.'));
                        new_arrangements.push(format!("{}{}", arr, '#'));
                    }
                }
                _ => panic!("invalid char"),
            }
            arrangements = new_arrangements;
        }
        arrangements
    }

    let mut num_arrangements = 0;

    for (springs, sequence) in conditions {
        for arrangement in generate_arrangements(springs) {
            let trimmed_arrangement = arrangement.trim_matches('.');
            let arrangement_sequence = trimmed_arrangement
                .split('.')
                .filter(|s| !s.is_empty())
                .map(|s| s.len() as u32)
                .collect::<Vec<u32>>();
            if arrangement_sequence == sequence {
                num_arrangements += 1;
            }
        }
    }

    Some(num_arrangements)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split('\n');
    let mut conditions = vec![];
    for line in lines {
        let [springs, sequence_str] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("could not parse line")
        };
        let mut sequence = vec![];
        for num_str in sequence_str.split(',') {
            sequence.push(num_str.parse::<u32>().unwrap_or(0));
        }
        let mut new_string = "".to_owned();
        let mut new_sequence = vec![];
        for i in 0..5 {
            if i != 0 {
                new_string += "?";
            }
            new_string += springs;
            new_sequence.extend(sequence.clone());
        }
        conditions.push((new_string, new_sequence));
    }
    println!("gotten here at least {:?}", conditions);

    fn generate_arrangements(original: String, sequence: &Vec<u32>) -> Vec<String> {
        let mut arrangements = vec![String::from("")];
        let mut subsequences = vec![];
        let mut curr_sequence = vec![];
        for s in sequence {
            curr_sequence.push(*s);
            subsequences.push(curr_sequence.clone());
        }
        for c in original.chars() {
            let mut new_arrangements = vec![];
            match c {
                '.' => {
                    for arr in arrangements {
                        new_arrangements.push(format!("{}{}", arr, c));
                    }
                }
                '#' => {
                    for arr in arrangements {
                        new_arrangements.push(format!("{}{}", arr, c));
                    }
                }
                '?' => {
                    for arr in arrangements {
                        new_arrangements.push(format!("{}{}", arr, '.'));
                        new_arrangements.push(format!("{}{}", arr, '#'));
                    }
                }
                _ => panic!("invalid char"),
            }
            new_arrangements = new_arrangements
                .iter()
                .filter(|arrangement| {
                    let trimmed_arrangement = arrangement.trim_matches('.');
                    let mut arrangement_sequence = trimmed_arrangement
                        .split('.')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.len() as u32)
                        .collect::<Vec<u32>>();
                    if arrangement_sequence[..].len() == 1 || arrangement_sequence[..].is_empty() {
                        return true;
                    }
                    if let Some(valid_sequence) = subsequences
                        .iter()
                        .find(|ss| ss.len() == arrangement_sequence.len() - 1)
                    {
                        arrangement_sequence.pop();
                        arrangement_sequence == *valid_sequence
                    } else {
                        false
                    }
                })
                .cloned()
                .collect::<Vec<String>>();
            arrangements = new_arrangements;
        }
        arrangements
    }

    let mut num_arrangements = 0;

    for (springs, sequence) in conditions {
        for arrangement in generate_arrangements(springs, &sequence) {
            let trimmed_arrangement = arrangement.trim_matches('.');
            let arrangement_sequence = trimmed_arrangement
                .split('.')
                .filter(|s| !s.is_empty())
                .map(|s| s.len() as u32)
                .collect::<Vec<u32>>();
            if arrangement_sequence == sequence {
                num_arrangements += 1;
            }
        }
    }

    Some(num_arrangements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        //        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //        assert_eq!(result, Some(525152));

        let result = part_two(".??..??...?##. 1,1,3");
        assert_eq!(result, Some(16384));
    }
}
