use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    #[derive(Clone)]
    struct Node {
        pos: (usize, usize),
        connected: Vec<(usize, usize)>,
    }
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut nodes = vec![];
    let mut start = Node {
        pos: (0, 0),
        connected: vec![],
    };
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let mut connected = vec![];
            match c {
                '|' => {
                    if i > 0 && lines[i - 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i - 1, j));
                    }
                    if i + 1 < lines.len() && lines[i + 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i + 1, j));
                    }
                }
                '-' => {
                    if j > 0 && lines[i].chars().nth(j - 1).unwrap_or('.') != '.' {
                        connected.push((i, j - 1));
                    }
                    if j + 1 < line.len() && lines[i].chars().nth(j + 1).unwrap_or('.') != '.' {
                        connected.push((i, j + 1));
                    }
                }
                'L' => {
                    if i > 0 && lines[i - 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i - 1, j));
                    }
                    if j + 1 < line.len() && lines[i].chars().nth(j + 1).unwrap_or('.') != '.' {
                        connected.push((i, j + 1));
                    }
                }
                'J' => {
                    if i > 0 && lines[i - 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i - 1, j));
                    }
                    if j > 0 && lines[i].chars().nth(j - 1).unwrap_or('.') != '.' {
                        connected.push((i, j - 1));
                    }
                }
                '7' => {
                    if i + 1 < lines.len() && lines[i + 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i + 1, j));
                    }
                    if j > 0 && lines[i].chars().nth(j - 1).unwrap_or('.') != '.' {
                        connected.push((i, j - 1));
                    }
                }
                'F' => {
                    if i + 1 < lines.len() && lines[i + 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i + 1, j));
                    }
                    if j + 1 < line.len() && lines[i].chars().nth(j + 1).unwrap_or('.') != '.' {
                        connected.push((i, j + 1));
                    }
                }
                'S' => {
                    start.pos = (i, j);
                }
                _ => {}
            };
            if c != 'S' {
                let node = Node {
                    pos: (i, j),
                    connected,
                };
                nodes.push(node);
            }
        }
    }
    start.connected = nodes
        .iter()
        .cloned()
        .filter(|node| node.connected.contains(&start.pos))
        .map(|node| node.clone().pos)
        .collect::<Vec<(usize, usize)>>();
    let start_pos = start.pos;
    nodes.push(start);
    let mut max_distance = 0;
    let mut seen = HashSet::new();
    let mut to_check = VecDeque::new();
    to_check.push_back((start_pos, 0));
    while let Some((pos, distance)) = to_check.pop_front() {
        if distance > max_distance {
            max_distance = distance;
        }
        let node = nodes.iter().find(|node| node.pos == pos).unwrap();
        for next_pos in &node.connected {
            if !seen.contains(next_pos) {
                to_check.push_back((*next_pos, distance + 1));
            }
        }
        seen.insert(pos);
    }
    Some(max_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    #[derive(Clone)]
    struct Node {
        pos: (usize, usize),
        connected: Vec<(usize, usize)>,
        symbol: char,
    }
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut nodes = vec![];
    let mut start = Node {
        pos: (0, 0),
        connected: vec![],
        symbol: 'S',
    };
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let mut connected = vec![];
            match c {
                '|' => {
                    if i > 0 && lines[i - 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i - 1, j));
                    }
                    if i + 1 < lines.len() && lines[i + 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i + 1, j));
                    }
                }
                '-' => {
                    if j > 0 && lines[i].chars().nth(j - 1).unwrap_or('.') != '.' {
                        connected.push((i, j - 1));
                    }
                    if j + 1 < line.len() && lines[i].chars().nth(j + 1).unwrap_or('.') != '.' {
                        connected.push((i, j + 1));
                    }
                }
                'L' => {
                    if i > 0 && lines[i - 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i - 1, j));
                    }
                    if j + 1 < line.len() && lines[i].chars().nth(j + 1).unwrap_or('.') != '.' {
                        connected.push((i, j + 1));
                    }
                }
                'J' => {
                    if i > 0 && lines[i - 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i - 1, j));
                    }
                    if j > 0 && lines[i].chars().nth(j - 1).unwrap_or('.') != '.' {
                        connected.push((i, j - 1));
                    }
                }
                '7' => {
                    if i + 1 < lines.len() && lines[i + 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i + 1, j));
                    }
                    if j > 0 && lines[i].chars().nth(j - 1).unwrap_or('.') != '.' {
                        connected.push((i, j - 1));
                    }
                }
                'F' => {
                    if i + 1 < lines.len() && lines[i + 1].chars().nth(j).unwrap_or('.') != '.' {
                        connected.push((i + 1, j));
                    }
                    if j + 1 < line.len() && lines[i].chars().nth(j + 1).unwrap_or('.') != '.' {
                        connected.push((i, j + 1));
                    }
                }
                'S' => {
                    start.pos = (i, j);
                }
                _ => {}
            };
            if c != 'S' {
                let node = Node {
                    pos: (i, j),
                    connected,
                    symbol: c,
                };
                nodes.push(node);
            }
        }
    }
    start.connected = nodes
        .iter()
        .cloned()
        .filter(|node| node.connected.contains(&start.pos))
        .map(|node| node.clone().pos)
        .collect::<Vec<(usize, usize)>>();
    let start_pos = start.pos;
    let config = [
        start.connected.contains(&(start_pos.0 - 1, start_pos.1)),
        start.connected.contains(&(start_pos.0 + 1, start_pos.1)),
        start.connected.contains(&(start_pos.0, start_pos.1 - 1)),
        start.connected.contains(&(start_pos.0, start_pos.1 + 1)),
    ];
    match config {
        [true, true, false, false] => start.symbol = '|',
        [true, false, true, false] => start.symbol = 'J',
        [true, false, false, true] => start.symbol = 'L',
        [false, true, true, false] => start.symbol = '7',
        [false, true, false, true] => start.symbol = 'F',
        [false, false, true, true] => start.symbol = '-',
        _ => panic!("unknown case"),
    }
    nodes.push(start);
    let mut seen = HashSet::new();
    let mut to_check = vec![];
    to_check.push(start_pos);
    let mut cycle_found = 0;
    while cycle_found != 2 {
        let pos = to_check.pop().unwrap();
        let node = nodes.iter().find(|node| node.pos == pos).unwrap();
        for next_pos in &node.connected {
            if !seen.contains(next_pos) {
                to_check.push(*next_pos);
            } else if *next_pos == start_pos {
                cycle_found += 1;
            }
        }
        seen.insert(pos);
    }
    let mut status = HashMap::new();
    let mut num = 0;
    for node in &nodes {
        if !seen.contains(&node.pos) {
            let mut crossings = 0;
            for i in node.pos.0..lines.len() {
                if seen.contains(&(i, node.pos.1)) {
                    let other = nodes
                        .iter()
                        .find(|el| el.pos.0 == i && el.pos.1 == node.pos.1)
                        .unwrap();
                    if "FL-".contains(other.symbol) {
                        crossings += 1;
                    }
                }
            }
            if crossings % 2 != 0 {
                num += 1;
                status.insert(node.pos, 'I');
            } else {
                status.insert(node.pos, 'O');
            }
        } else {
            status.insert(node.pos, node.symbol);
        }
    }
    Some(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let mut input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let mut result = part_two(input);
        assert_eq!(result, Some(4));
        input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        result = part_two(input);
        assert_eq!(result, Some(8));
    }
}
