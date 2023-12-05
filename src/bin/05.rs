advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let [seeds_str, seed_to_soil_str, soil_to_fertilizer_str, fertilizer_to_water_str, water_to_light_str, light_to_temperature_str, temperature_to_humidity_str, humidity_to_location_str] =
        input.split("\n\n").collect::<Vec<&str>>()[..]
    else {
        panic!("could not unpack sections correctly");
    };
    let seeds_split = &seeds_str.split(' ').collect::<Vec<&str>>()[1..];
    let seeds = seeds_split
        .iter()
        .map(|seed| seed.parse::<u64>().unwrap_or(0));
    let mut seed_to_soil_range = vec![];
    for line in &mut seed_to_soil_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        seed_to_soil_range.push((source_range, dest_range));
    }
    let mut soil_to_fertilizer_range = vec![];
    for line in &mut soil_to_fertilizer_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        soil_to_fertilizer_range.push((source_range, dest_range));
    }
    let mut fertilizer_to_water_range = vec![];
    for line in &mut fertilizer_to_water_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..source_start + length;
        let dest_range = dest_start..dest_start + length;
        fertilizer_to_water_range.push((source_range, dest_range));
    }
    let mut water_to_light_range = vec![];
    for line in &mut water_to_light_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        water_to_light_range.push((source_range, dest_range));
    }
    let mut light_to_temperature_range = vec![];
    for line in &mut light_to_temperature_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        light_to_temperature_range.push((source_range, dest_range));
    }
    let mut temperature_to_humidity_range = vec![];
    for line in &mut temperature_to_humidity_str
        .split('\n')
        .collect::<Vec<&str>>()[1..]
    {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        temperature_to_humidity_range.push((source_range, dest_range));
    }
    let mut humidity_to_location_range = vec![];
    for line in &mut humidity_to_location_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        humidity_to_location_range.push((source_range, dest_range));
    }

    let locations = seeds.map(|seed| {
        let mut maybe_soil = None;
        for (source_range, dest_range) in &seed_to_soil_range {
            if source_range.contains(&seed) {
                let offset = seed - source_range.start;
                maybe_soil = Some(dest_range.start + offset);
            }
        }
        if maybe_soil.is_none() {
            maybe_soil = Some(seed);
        }
        let soil = maybe_soil.unwrap_or(0);
        let mut maybe_fertilizer = None;
        for (source_range, dest_range) in &soil_to_fertilizer_range {
            if source_range.contains(&soil) {
                let offset = soil - source_range.start;
                maybe_fertilizer = Some(dest_range.start + offset);
            }
        }
        if maybe_fertilizer.is_none() {
            maybe_fertilizer = Some(soil);
        }
        let fertilizer = maybe_fertilizer.unwrap_or(0);
        let mut maybe_water = None;
        for (source_range, dest_range) in &fertilizer_to_water_range {
            if source_range.contains(&fertilizer) {
                let offset = fertilizer - source_range.start;
                maybe_water = Some(dest_range.start + offset);
            }
        }
        if maybe_water.is_none() {
            maybe_water = Some(fertilizer);
        }
        let water = maybe_water.unwrap_or(0);
        let mut maybe_light = None;
        for (source_range, dest_range) in &water_to_light_range {
            if source_range.contains(&water) {
                let offset = water - source_range.start;
                maybe_light = Some(dest_range.start + offset);
            }
        }
        if maybe_light.is_none() {
            maybe_light = Some(water);
        }
        let light = maybe_light.unwrap_or(0);
        let mut maybe_temperature = None;
        for (source_range, dest_range) in &light_to_temperature_range {
            if source_range.contains(&light) {
                let offset = light - source_range.start;
                maybe_temperature = Some(dest_range.start + offset);
            }
        }
        if maybe_temperature.is_none() {
            maybe_temperature = Some(light);
        }
        let temperature = maybe_temperature.unwrap_or(0);
        let mut maybe_humidity = None;
        for (source_range, dest_range) in &temperature_to_humidity_range {
            if source_range.contains(&temperature) {
                let offset = temperature - source_range.start;
                maybe_humidity = Some(dest_range.start + offset);
            }
        }
        if maybe_humidity.is_none() {
            maybe_humidity = Some(temperature);
        }
        let humidity = maybe_humidity.unwrap_or(0);
        let mut maybe_location = None;
        for (source_range, dest_range) in &humidity_to_location_range {
            if source_range.contains(&humidity) {
                let offset = humidity - source_range.start;
                maybe_location = Some(dest_range.start + offset);
            }
        }
        if maybe_location.is_none() {
            maybe_location = Some(humidity);
        }
        maybe_location.unwrap_or(0)
    });
    locations.min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let [seeds_str, seed_to_soil_str, soil_to_fertilizer_str, fertilizer_to_water_str, water_to_light_str, light_to_temperature_str, temperature_to_humidity_str, humidity_to_location_str] =
        input.split("\n\n").collect::<Vec<&str>>()[..]
    else {
        panic!("could not unpack sections correctly");
    };
    let seeds_split = &seeds_str.split(' ').collect::<Vec<&str>>()[1..];
    let mut seeds = vec![];
    let seed_input = seeds_split
        .iter()
        .map(|seed| seed.parse::<u64>().unwrap_or(0))
        .collect::<Vec<u64>>();
    for i in (0..seed_input.len()).step_by(2) {
        seeds.push(seed_input[i]..(seed_input[i] + seed_input[i + 1]));
    }
    let mut seed_to_soil_range = vec![];
    for line in &mut seed_to_soil_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        seed_to_soil_range.push((source_range, dest_range));
    }
    let mut soil_to_fertilizer_range = vec![];
    for line in &mut soil_to_fertilizer_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        soil_to_fertilizer_range.push((source_range, dest_range));
    }
    let mut fertilizer_to_water_range = vec![];
    for line in &mut fertilizer_to_water_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..source_start + length;
        let dest_range = dest_start..dest_start + length;
        fertilizer_to_water_range.push((source_range, dest_range));
    }
    let mut water_to_light_range = vec![];
    for line in &mut water_to_light_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        water_to_light_range.push((source_range, dest_range));
    }
    let mut light_to_temperature_range = vec![];
    for line in &mut light_to_temperature_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        light_to_temperature_range.push((source_range, dest_range));
    }
    let mut temperature_to_humidity_range = vec![];
    for line in &mut temperature_to_humidity_str
        .split('\n')
        .collect::<Vec<&str>>()[1..]
    {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        temperature_to_humidity_range.push((source_range, dest_range));
    }
    let mut humidity_to_location_range = vec![];
    for line in &mut humidity_to_location_str.split('\n').collect::<Vec<&str>>()[1..] {
        let [dest_start_str, source_start_str, length_str] =
            line.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!("could not unpack range builder correctly")
        };
        let source_start = source_start_str.parse::<u64>().unwrap_or(0);
        let dest_start = dest_start_str.parse::<u64>().unwrap_or(0);
        let length = length_str.parse::<u64>().unwrap_or(0);
        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        humidity_to_location_range.push((source_range, dest_range));
    }

    for location in 0..u64::MAX {
        let mut maybe_humidity = None;
        for (source_range, dest_range) in &humidity_to_location_range {
            if dest_range.contains(&location) {
                let offset = location - dest_range.start;
                maybe_humidity = Some(source_range.start + offset);
            }
        }
        if maybe_humidity.is_none() {
            maybe_humidity = Some(location);
        }
        let humidity = maybe_humidity.unwrap_or(0);
        let mut maybe_temperature = None;
        for (source_range, dest_range) in &temperature_to_humidity_range {
            if dest_range.contains(&humidity) {
                let offset = humidity - dest_range.start;
                maybe_temperature = Some(source_range.start + offset);
            }
        }
        if maybe_temperature.is_none() {
            maybe_temperature = Some(humidity);
        }
        let temperature = maybe_temperature.unwrap_or(0);
        let mut maybe_light = None;
        for (source_range, dest_range) in &light_to_temperature_range {
            if dest_range.contains(&temperature) {
                let offset = temperature - dest_range.start;
                maybe_light = Some(source_range.start + offset);
            }
        }
        if maybe_light.is_none() {
            maybe_light = Some(temperature);
        }
        let light = maybe_light.unwrap_or(0);
        let mut maybe_water = None;
        for (source_range, dest_range) in &water_to_light_range {
            if dest_range.contains(&light) {
                let offset = light - dest_range.start;
                maybe_water = Some(source_range.start + offset);
            }
        }
        if maybe_water.is_none() {
            maybe_water = Some(light);
        }
        let water = maybe_water.unwrap_or(0);
        let mut maybe_fertilizer = None;
        for (source_range, dest_range) in &fertilizer_to_water_range {
            if dest_range.contains(&water) {
                let offset = water - dest_range.start;
                maybe_fertilizer = Some(source_range.start + offset);
            }
        }
        if maybe_fertilizer.is_none() {
            maybe_fertilizer = Some(water);
        }
        let fertilizer = maybe_fertilizer.unwrap_or(0);
        let mut maybe_soil = None;
        for (source_range, dest_range) in &soil_to_fertilizer_range {
            if dest_range.contains(&fertilizer) {
                let offset = fertilizer - dest_range.start;
                maybe_soil = Some(source_range.start + offset);
            }
        }
        if maybe_soil.is_none() {
            maybe_soil = Some(fertilizer);
        }
        let soil = maybe_soil.unwrap_or(0);
        let mut maybe_seed = None;
        for (source_range, dest_range) in &seed_to_soil_range {
            if dest_range.contains(&soil) {
                let offset = soil - dest_range.start;
                maybe_seed = Some(source_range.start + offset);
            }
        }
        if maybe_seed.is_none() {
            maybe_seed = Some(soil);
        }
        let seed = maybe_seed.unwrap_or(0);
        for seed_range in &seeds {
            if seed_range.contains(&seed) {
                return Some(location);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
