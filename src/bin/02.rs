use regex::Regex;
advent_of_code::solution!(2);

struct Game {
    id: u32,
    subsets: Vec<(u32, u32, u32)>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .collect::<Vec<&str>>();
    let mut games = vec![];
    let line_re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    for line in lines {
        let (_, [id_text, subsets_text]) = line_re.captures(line).unwrap().extract();
        let mut subsets = vec![];
        let subsets_split = subsets_text.split("; ").collect::<Vec<&str>>();
        for subset in subsets_split {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let cubes = subset.split(", ").collect::<Vec<&str>>();
            for cube in cubes {
                let cube_pair = cube.split(' ').collect::<Vec<&str>>();
                let cube_slice = cube_pair.as_slice();
                match cube_slice {
                    [num, "red"] => {
                        r = num.parse::<u32>().unwrap_or(0);
                    }
                    [num, "green"] => {
                        g = num.parse::<u32>().unwrap_or(0);
                    }
                    [num, "blue"] => {
                        b = num.parse::<u32>().unwrap_or(0);
                    }
                    _ => {}
                };
                subsets.push((r, g, b));
            }
        }
        let game = Game {
            id: id_text.parse::<u32>().unwrap_or(0),
            subsets,
        };
        games.push(game);
    }
    let mut num_valid = 0;
    for game in games {
        let mut valid = true;
        for subset in &game.subsets {
            if subset.0 > 12 || subset.1 > 13 || subset.2 > 14 {
                valid = false;
            }
        }
        if valid {
            num_valid += game.id;
        }
    }
    Some(num_valid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .collect::<Vec<&str>>();
    let mut games = vec![];
    let line_re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    for line in lines {
        let (_, [id_text, subsets_text]) = line_re.captures(line).unwrap().extract();
        let mut subsets = vec![];
        let subsets_split = subsets_text.split("; ").collect::<Vec<&str>>();
        for subset in subsets_split {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let cubes = subset.split(", ").collect::<Vec<&str>>();
            for cube in cubes {
                let cube_pair = cube.split(' ').collect::<Vec<&str>>();
                let cube_slice = cube_pair.as_slice();
                match cube_slice {
                    [num, "red"] => {
                        r = num.parse::<u32>().unwrap_or(0);
                    }
                    [num, "green"] => {
                        g = num.parse::<u32>().unwrap_or(0);
                    }
                    [num, "blue"] => {
                        b = num.parse::<u32>().unwrap_or(0);
                    }
                    _ => {}
                };
                subsets.push((r, g, b));
            }
        }
        let game = Game {
            id: id_text.parse::<u32>().unwrap_or(0),
            subsets,
        };
        games.push(game);
    }
    let mut sum = 0;
    for game in games {
        let mut min_red = u32::MIN;
        let mut min_green = u32::MIN;
        let mut min_blue = u32::MIN;
        for subset in game.subsets {
            if subset.0 > min_red {
                min_red = subset.0;
            }
            if subset.1 > min_green {
                min_green = subset.1;
            }
            if subset.2 > min_blue {
                min_blue = subset.2;
            }
        }
        sum += min_red * min_green * min_blue;
    }
    Some(sum)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
