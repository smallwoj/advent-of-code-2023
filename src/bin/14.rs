use std::{char, collections::HashMap};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = vec![];
    for line in input.split('\n') {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_grid = vec![];
        for i in 0..grid.first().unwrap().len() {
            let mut new_row = vec![];
            for row in &grid {
                new_row.push(row[i]);
            }
            new_grid.push(new_row);
        }
        new_grid
    }
    grid = rotate(grid);
    let grid_height = grid.first().unwrap().len();
    let mut load = 0;
    for row in &grid {
        let mut num_stones = 0;
        let mut base_col = 0;
        for (j, c) in row.iter().enumerate() {
            match c {
                'O' => {
                    load += grid_height - base_col - num_stones;
                    num_stones += 1;
                }
                '#' => {
                    base_col = j + 1;
                    num_stones = 0;
                }
                _ => {}
            }
        }
    }
    Some(load)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = vec![];
    for line in input.split('\n') {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
        for r in 0..grid.len() {
            for (c, row) in new_grid.iter_mut().enumerate().take(grid[0].len()) {
                row[grid.len() - 1 - r] = grid[r][c];
            }
        }
        new_grid
    }
    fn tilt(grid: &mut Vec<Vec<char>>) {
        let mut done = false;
        while !done {
            done = true;
            for r in 0..grid.len() - 1 {
                for c in 0..grid[0].len() {
                    if grid[r + 1][c] == 'O' && grid[r][c] == '.' {
                        grid[r][c] = 'O';
                        grid[r + 1][c] = '.';
                        done = false;
                    }
                }
            }
        }
    }
    fn load(grid: Vec<Vec<char>>) -> usize {
        (0..grid.len())
            .map(|r| {
                (0..grid[0].len())
                    .filter(|&c| grid[r][c] == 'O')
                    .map(|_| grid.len() - r)
                    .sum::<usize>()
            })
            .sum()
    }
    let mut seen = HashMap::new();
    for n in 1..1000000000 {
        for _ in 0..4 {
            tilt(&mut grid);
            grid = rotate(grid);
        }
        if let Some(seen_at) = seen.insert(grid.clone(), n) {
            if (1000000000 - n) % (n - seen_at) == 0 {
                break;
            }
        }
    }
    Some(load(grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
