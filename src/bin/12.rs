advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').filter(|el| !el.is_empty());
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
    let lines = input.split('\n').filter(|el| !el.is_empty());
    let mut conditions = vec![];
    for line in lines {
        let [springs, sequence_str] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("could not parse line")
        };
        let mut sequence = vec![];
        for num_str in sequence_str.split(',') {
            sequence.push(num_str.parse::<usize>().unwrap_or(0));
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
        new_string += ".";
        conditions.push((new_string, new_sequence));
    }

    fn count_possible_arrangements(
        springs: &[char],
        counts: &[usize],
        cache: &mut [Vec<Option<u64>>],
    ) -> u64 {
        if counts.is_empty() {
            return if springs.contains(&'#') { 0 } else { 1 };
        }
        if springs.len() < counts.iter().sum::<usize>() + counts.len() {
            return 0;
        }
        if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
            return cached;
        }
        let mut arrangements = 0;
        if springs[0] != '#' {
            arrangements += count_possible_arrangements(&springs[1..], counts, cache);
        }
        let next_group_size = counts[0];
        if !springs[..next_group_size].contains(&'.') && springs[next_group_size] != '#' {
            arrangements +=
                count_possible_arrangements(&springs[next_group_size + 1..], &counts[1..], cache);
        }
        cache[counts.len() - 1][springs.len() - 1] = Some(arrangements);
        arrangements
    }

    let mut num_arrangements = 0;

    for (springs, sequence) in conditions {
        let mut cache = vec![vec![None; springs.len()]; sequence.len()];
        num_arrangements += count_possible_arrangements(
            &springs.chars().collect::<Vec<_>>()[..],
            &sequence[..],
            &mut cache,
        );
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

        let mut result = part_two("???.### 1,1,3");
        assert_eq!(result, Some(1));
        result = part_two(".??..??...?##. 1,1,3");
        assert_eq!(result, Some(16384));
        result = part_two("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result, Some(1));
        result = part_two("????.#...#... 4,1,1");
        assert_eq!(result, Some(16));
        result = part_two("????.######..#####. 1,6,5");
        assert_eq!(result, Some(2500));
        result = part_two("?###???????? 3,2,1");
        assert_eq!(result, Some(506250));
    }
}
