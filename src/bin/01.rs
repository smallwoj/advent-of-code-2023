advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut line_nums = vec![];
    for line in lines {
        let mut first_num = '0';
        let mut last_num = '0';
        for c in line.chars() {
            if "123456789".contains(c) {
                last_num = c;
            }
        }
        for c in line.chars().rev() {
            if "123456789".contains(c) {
                first_num = c;
            }
        }
        let str_num = format!("{}{}", first_num, last_num);
        line_nums.push(str_num.parse::<u32>().unwrap_or(0))
    }
    Some(line_nums.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let input_strings = lines.iter().map(|x| x.to_string());
    let mut line_nums = vec![];
    for line in input_strings {
        let word_nums = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];
        let mut final_string = String::from("");
        let mut c = 0;
        while c < line.len() {
            let mut found = false;
            for (word, num) in word_nums {
                if c + word.len() <= line.len() && &line[c..(c + word.len())] == word {
                    final_string.push_str(num);
                    c += word.len() - 1;
                    found = true;
                }
            }
            if !found {
                final_string.push(line.chars().nth(c).unwrap_or('0'));
                c += 1;
            }
        }
        let mut first_num = '0';
        let mut last_num = '0';
        for c in final_string.chars() {
            if "123456789".contains(c) {
                last_num = c;
            }
        }
        for c in final_string.chars().rev() {
            if "123456789".contains(c) {
                first_num = c;
            }
        }
        let str_num = format!("{}{}", first_num, last_num);
        line_nums.push(str_num.parse::<u32>().unwrap_or(0))
    }
    Some(line_nums.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
