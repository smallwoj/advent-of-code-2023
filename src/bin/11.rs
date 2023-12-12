use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.split('\n');
    let mut empty_rows = HashSet::new();
    let mut galaxies = vec![];
    for (i, line) in lines.enumerate() {
        let mut galaxy_found = false;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
                galaxy_found = true;
            }
        }
        if !galaxy_found {
            empty_rows.insert(i);
        }
    }
    let mut empty_cols = (0..input.split('\n').next().unwrap().len()).collect::<HashSet<usize>>();
    for galaxy in &galaxies {
        if empty_cols.contains(&galaxy.1) {
            empty_cols.remove(&galaxy.1);
        }
    }
    let mut pairs = vec![];
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(i + 1) {
            pairs.push((galaxy1, galaxy2));
        }
    }
    let mut total_distance = 0;
    for (galaxy1, galaxy2) in pairs {
        let mut num_rows = 0;
        let mut num_cols = 0;
        for row in &empty_rows {
            if &galaxy1.0 < row && row < &galaxy2.0 || &galaxy1.0 > row && row > &galaxy2.0 {
                num_rows += 1;
            }
        }
        for col in &empty_cols {
            if &galaxy1.1 < col && col < &galaxy2.1 || &galaxy1.1 > col && col > &galaxy2.1 {
                num_cols += 1;
            }
        }

        let mut distance = (galaxy1.0 as i32 - galaxy2.0 as i32).abs()
            + (galaxy1.1 as i32 - galaxy2.1 as i32).abs();
        distance += num_cols + num_rows;
        total_distance += distance;
    }
    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.split('\n');
    let mut empty_rows = HashSet::new();
    let mut galaxies = vec![];
    for (i, line) in lines.enumerate() {
        let mut galaxy_found = false;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
                galaxy_found = true;
            }
        }
        if !galaxy_found {
            empty_rows.insert(i);
        }
    }
    let mut empty_cols = (0..input.split('\n').next().unwrap().len()).collect::<HashSet<usize>>();
    for galaxy in &galaxies {
        if empty_cols.contains(&galaxy.1) {
            empty_cols.remove(&galaxy.1);
        }
    }
    let mut pairs = vec![];
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(i + 1) {
            pairs.push((galaxy1, galaxy2));
        }
    }
    let mut total_distance = 0;
    for (galaxy1, galaxy2) in pairs {
        let mut num_rows = 0;
        let mut num_cols = 0;
        for row in &empty_rows {
            if &galaxy1.0 < row && row < &galaxy2.0 || &galaxy1.0 > row && row > &galaxy2.0 {
                num_rows += 1;
            }
        }
        for col in &empty_cols {
            if &galaxy1.1 < col && col < &galaxy2.1 || &galaxy1.1 > col && col > &galaxy2.1 {
                num_cols += 1;
            }
        }

        let mut distance = (galaxy1.0 as i64 - galaxy2.0 as i64).abs()
            + (galaxy1.1 as i64 - galaxy2.1 as i64).abs();
        distance += (num_cols + num_rows) * (1000000 - 1);
        total_distance += distance;
    }
    Some(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
