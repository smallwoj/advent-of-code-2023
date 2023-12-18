advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<i32> {
    fn find_horizontal_line_of_reflection(lines: Vec<String>) -> Option<i32> {
        let mut line_index = None;
        for i in 0..lines.len() as i32 - 1 {
            let mut j: i32 = 0;
            let mut valid = true;
            while i - j >= 0 && i + j + 1 < lines.len() as i32 && valid {
                valid = lines[(i - j) as usize] == lines[(i + j + 1) as usize];
                j += 1;
            }
            if valid {
                line_index = Some(i);
                break;
            }
        }
        line_index
    }
    Some(
        input
            .split("\n\n")
            .map(|grid_str| {
                let lines = grid_str
                    .split('\n')
                    .map(|str| str.to_owned())
                    .collect::<Vec<String>>();
                let line_index = find_horizontal_line_of_reflection(lines.clone());
                if let Some(horizontal_line_index) = line_index {
                    100 * (horizontal_line_index + 1)
                } else {
                    let mut new_lines = vec![];
                    for i in 0..lines.first().unwrap().len() {
                        let mut new_line = String::from("");
                        for line in &lines {
                            new_line = format!("{}{}", new_line, line.chars().nth(i).unwrap());
                        }
                        new_lines.push(new_line);
                    }
                    let new_line_index = find_horizontal_line_of_reflection(new_lines);
                    new_line_index.unwrap() + 1
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    fn find_horizontal_line_of_reflection(
        lines: Vec<String>,
        original_index: Option<i32>,
    ) -> Option<i32> {
        let mut line_index = None;
        for i in 0..lines.len() as i32 - 1 {
            let mut j: i32 = 0;
            let mut valid = true;
            while i - j >= 0 && i + j + 1 < lines.len() as i32 && valid {
                valid = lines[(i - j) as usize] == lines[(i + j + 1) as usize];
                j += 1;
            }
            if valid && Some(i) != original_index {
                line_index = Some(i);
                break;
            }
        }
        line_index
    }

    Some(
        input
            .split("\n\n")
            .map(|grid_str| {
                let lines = grid_str
                    .split('\n')
                    .map(|str| str.to_owned())
                    .collect::<Vec<String>>();
                let mut original_horizontal = false;
                let line_index = find_horizontal_line_of_reflection(lines.clone(), None);
                let original_index = if let Some(horizontal_line_index) = line_index {
                    original_horizontal = true;
                    horizontal_line_index
                } else {
                    let mut new_lines = vec![];
                    for i in 0..lines.first().unwrap().len() {
                        let mut new_line = String::from("");
                        for line in &lines {
                            new_line = format!("{}{}", new_line, line.chars().nth(i).unwrap());
                        }
                        new_lines.push(new_line);
                    }
                    let new_line_index = find_horizontal_line_of_reflection(new_lines, None);
                    new_line_index.unwrap()
                };
                for i in 0..lines.len() {
                    for j in 0..lines[i].len() {
                        let mut unsmudged_lines = vec![];
                        for (k, line) in lines.iter().enumerate() {
                            let mut new_line = String::from("");
                            for (l, c) in line.chars().enumerate() {
                                let char_to_add = if i == k && j == l {
                                    if c == '.' {
                                        '#'
                                    } else {
                                        '.'
                                    }
                                } else {
                                    c
                                };
                                new_line = format!("{}{}", new_line, char_to_add);
                            }
                            unsmudged_lines.push(new_line);
                        }
                        let smudged_index = find_horizontal_line_of_reflection(
                            unsmudged_lines.clone(),
                            if original_horizontal {
                                Some(original_index)
                            } else {
                                None
                            },
                        );
                        if let Some(smudged_line_index) = smudged_index {
                            return 100 * (smudged_line_index + 1);
                        } else {
                            let mut new_smudged_lines = vec![];
                            for i in 0..unsmudged_lines.first().unwrap().len() {
                                let mut new_line = String::from("");
                                for line in &unsmudged_lines {
                                    new_line =
                                        format!("{}{}", new_line, line.chars().nth(i).unwrap());
                                }
                                new_smudged_lines.push(new_line);
                            }
                            let new_smudged_line_index = find_horizontal_line_of_reflection(
                                new_smudged_lines,
                                if !original_horizontal {
                                    Some(original_index)
                                } else {
                                    None
                                },
                            );
                            if let Some(vertical_index) = new_smudged_line_index {
                                return vertical_index + 1;
                            }
                        }
                    }
                }
                0
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
