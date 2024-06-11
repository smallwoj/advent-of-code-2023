use std::collections::HashSet;

advent_of_code::solution!(16);

#[derive(Debug)]
enum Tile {
    Empty,
    VerticalMirror,
    HorizontalMirror,
    SlashMirror,
    BackslashMirror,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = vec![];
    for line in input.split('\n').filter(|el| !el.is_empty()) {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                '.' => Tile::Empty,
                '/' => Tile::SlashMirror,
                '\\' => Tile::BackslashMirror,
                '|' => Tile::VerticalMirror,
                '-' => Tile::HorizontalMirror,
                _ => panic!("unexpected tile {}", c),
            });
        }
        grid.push(row);
    }

    let mut seen = HashSet::new();
    let mut energized = HashSet::new();
    let mut to_check = vec![((0, 0), Dir::Right)];
    while let Some((pos, dir)) = to_check.pop() {
        if !seen.contains(&(pos, dir)) && (pos.0 < grid.len() && pos.1 < grid[0].len()) {
            match grid[pos.0][pos.1] {
                Tile::Empty => {
                    if let (Some(r), Some(c)) = match dir {
                        Dir::Up => (pos.0.checked_sub(1), Some(pos.1)),
                        Dir::Left => (Some(pos.0), pos.1.checked_sub(1)),
                        Dir::Down => (Some(pos.0 + 1), Some(pos.1)),
                        Dir::Right => (Some(pos.0), Some(pos.1 + 1)),
                    } {
                        to_check.push(((r, c), dir));
                    }
                }
                Tile::SlashMirror => {
                    if let (Some(r), Some(c)) = match dir {
                        Dir::Up => (pos.0.checked_add(0), pos.1.checked_add(1)),
                        Dir::Left => (pos.0.checked_add(1), Some(pos.1)),
                        Dir::Down => (Some(pos.0), pos.1.checked_sub(1)),
                        Dir::Right => (pos.0.checked_sub(1), Some(pos.1)),
                    } {
                        to_check.push((
                            (r, c),
                            match dir {
                                Dir::Up => Dir::Right,
                                Dir::Left => Dir::Down,
                                Dir::Down => Dir::Left,
                                Dir::Right => Dir::Up,
                            },
                        ));
                    }
                }
                Tile::BackslashMirror => {
                    if let (Some(r), Some(c)) = match dir {
                        Dir::Up => (pos.0.checked_add(0), pos.1.checked_sub(1)),
                        Dir::Left => (pos.0.checked_sub(1), Some(pos.1)),
                        Dir::Down => (Some(pos.0), pos.1.checked_add(1)),
                        Dir::Right => (pos.0.checked_add(1), Some(pos.1)),
                    } {
                        to_check.push((
                            (r, c),
                            match dir {
                                Dir::Up => Dir::Left,
                                Dir::Left => Dir::Up,
                                Dir::Down => Dir::Right,
                                Dir::Right => Dir::Down,
                            },
                        ));
                    }
                }
                Tile::VerticalMirror => match dir {
                    Dir::Right | Dir::Left => {
                        to_check.push((pos, Dir::Up));
                        to_check.push((pos, Dir::Down));
                    }
                    Dir::Up | Dir::Down => {
                        if let (Some(r), Some(c)) = match dir {
                            Dir::Up => (pos.0.checked_sub(1), Some(pos.1)),
                            Dir::Left => (Some(pos.0), pos.1.checked_sub(1)),
                            Dir::Down => (Some(pos.0 + 1), Some(pos.1)),
                            Dir::Right => (Some(pos.0), Some(pos.1 + 1)),
                        } {
                            to_check.push(((r, c), dir))
                        }
                    }
                },
                Tile::HorizontalMirror => match dir {
                    Dir::Up | Dir::Down => {
                        to_check.push((pos, Dir::Left));
                        to_check.push((pos, Dir::Right));
                    }
                    Dir::Right | Dir::Left => {
                        if let (Some(r), Some(c)) = match dir {
                            Dir::Up => (pos.0.checked_sub(1), Some(pos.1)),
                            Dir::Left => (Some(pos.0), pos.1.checked_sub(1)),
                            Dir::Down => (Some(pos.0 + 1), Some(pos.1)),
                            Dir::Right => (Some(pos.0), Some(pos.1 + 1)),
                        } {
                            to_check.push(((r, c), dir))
                        }
                    }
                },
            }
            seen.insert((pos, dir));
            energized.insert(pos);
        }
    }
    Some(energized.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = vec![];
    for line in input.split('\n').filter(|el| !el.is_empty()) {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                '.' => Tile::Empty,
                '/' => Tile::SlashMirror,
                '\\' => Tile::BackslashMirror,
                '|' => Tile::VerticalMirror,
                '-' => Tile::HorizontalMirror,
                _ => panic!("unexpected tile {}", c),
            });
        }
        grid.push(row);
    }
    let mut starts = vec![];
    for r in 0..grid.len() {
        starts.push(((r, 0), Dir::Right));
        starts.push(((r, grid[0].len() - 1), Dir::Left));
    }
    for c in 0..grid[0].len() {
        starts.push(((0, c), Dir::Down));
        starts.push(((grid.len() - 1, c), Dir::Up));
    }
    Some(
        starts
            .iter()
            .map(|&start| {
                let mut seen = HashSet::new();
                let mut energized = HashSet::new();
                let mut to_check = vec![start];
                while let Some((pos, dir)) = to_check.pop() {
                    if !seen.contains(&(pos, dir)) && (pos.0 < grid.len() && pos.1 < grid[0].len())
                    {
                        match grid[pos.0][pos.1] {
                            Tile::Empty => {
                                if let (Some(r), Some(c)) = match dir {
                                    Dir::Up => (pos.0.checked_sub(1), Some(pos.1)),
                                    Dir::Left => (Some(pos.0), pos.1.checked_sub(1)),
                                    Dir::Down => (Some(pos.0 + 1), Some(pos.1)),
                                    Dir::Right => (Some(pos.0), Some(pos.1 + 1)),
                                } {
                                    to_check.push(((r, c), dir));
                                }
                            }
                            Tile::SlashMirror => {
                                if let (Some(r), Some(c)) = match dir {
                                    Dir::Up => (pos.0.checked_add(0), pos.1.checked_add(1)),
                                    Dir::Left => (pos.0.checked_add(1), Some(pos.1)),
                                    Dir::Down => (Some(pos.0), pos.1.checked_sub(1)),
                                    Dir::Right => (pos.0.checked_sub(1), Some(pos.1)),
                                } {
                                    to_check.push((
                                        (r, c),
                                        match dir {
                                            Dir::Up => Dir::Right,
                                            Dir::Left => Dir::Down,
                                            Dir::Down => Dir::Left,
                                            Dir::Right => Dir::Up,
                                        },
                                    ));
                                }
                            }
                            Tile::BackslashMirror => {
                                if let (Some(r), Some(c)) = match dir {
                                    Dir::Up => (pos.0.checked_add(0), pos.1.checked_sub(1)),
                                    Dir::Left => (pos.0.checked_sub(1), Some(pos.1)),
                                    Dir::Down => (Some(pos.0), pos.1.checked_add(1)),
                                    Dir::Right => (pos.0.checked_add(1), Some(pos.1)),
                                } {
                                    to_check.push((
                                        (r, c),
                                        match dir {
                                            Dir::Up => Dir::Left,
                                            Dir::Left => Dir::Up,
                                            Dir::Down => Dir::Right,
                                            Dir::Right => Dir::Down,
                                        },
                                    ));
                                }
                            }
                            Tile::VerticalMirror => match dir {
                                Dir::Right | Dir::Left => {
                                    to_check.push((pos, Dir::Up));
                                    to_check.push((pos, Dir::Down));
                                }
                                Dir::Up | Dir::Down => {
                                    if let (Some(r), Some(c)) = match dir {
                                        Dir::Up => (pos.0.checked_sub(1), Some(pos.1)),
                                        Dir::Left => (Some(pos.0), pos.1.checked_sub(1)),
                                        Dir::Down => (Some(pos.0 + 1), Some(pos.1)),
                                        Dir::Right => (Some(pos.0), Some(pos.1 + 1)),
                                    } {
                                        to_check.push(((r, c), dir))
                                    }
                                }
                            },
                            Tile::HorizontalMirror => match dir {
                                Dir::Up | Dir::Down => {
                                    to_check.push((pos, Dir::Left));
                                    to_check.push((pos, Dir::Right));
                                }
                                Dir::Right | Dir::Left => {
                                    if let (Some(r), Some(c)) = match dir {
                                        Dir::Up => (pos.0.checked_sub(1), Some(pos.1)),
                                        Dir::Left => (Some(pos.0), pos.1.checked_sub(1)),
                                        Dir::Down => (Some(pos.0 + 1), Some(pos.1)),
                                        Dir::Right => (Some(pos.0), Some(pos.1 + 1)),
                                    } {
                                        to_check.push(((r, c), dir))
                                    }
                                }
                            },
                        }
                        seen.insert((pos, dir));
                        energized.insert(pos);
                    }
                }
                energized.len()
            })
            .max()
            .unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
