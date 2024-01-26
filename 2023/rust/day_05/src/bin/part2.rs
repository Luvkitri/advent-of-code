use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
enum MappingType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl MappingType {
    const DEFAULT_MAPPING_ORDER: [Self; 7] = [
        Self::SeedToSoil,
        Self::SoilToFertilizer,
        Self::FertilizerToWater,
        Self::WaterToLight,
        Self::LightToTemperature,
        Self::TemperatureToHumidity,
        Self::HumidityToLocation,
    ];
}

#[derive(Copy, Clone, Debug)]
struct MappingParameters {
    source_range_start: i64,
    destination_range_start: i64,
    range_length: i64,
}

#[derive(Copy, Clone, Debug)]
struct KeyRange {
    start: i64,
    length: i64,
}

fn main() {
    let input = include_str!("./input1.txt").trim();
    let (seed_ranges, mapping_data) = parse_input(input);
    let output = get_lowest_location(seed_ranges, mapping_data);
    dbg!(output);
}

fn parse_seeds(section: &str) -> Vec<KeyRange> {
    let re = Regex::new(r"\d+").unwrap();
    let seed_parameters: Vec<i64> = re
        .find_iter(section)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();

    let mut seed_ranges = Vec::new();
    for i in (0..seed_parameters.len()).step_by(2) {
        seed_ranges.push(KeyRange {
            start: *seed_parameters.get(i).unwrap(),
            length: *seed_parameters.get(i + 1).unwrap(),
        });
    }

    seed_ranges
}

fn parse_mapping_type(map: &str) -> Option<MappingType> {
    match map {
        "seed-to-soil map:" => Some(MappingType::SeedToSoil),
        "soil-to-fertilizer map:" => Some(MappingType::SoilToFertilizer),
        "fertilizer-to-water map:" => Some(MappingType::FertilizerToWater),
        "water-to-light map:" => Some(MappingType::WaterToLight),
        "light-to-temperature map:" => Some(MappingType::LightToTemperature),
        "temperature-to-humidity map:" => Some(MappingType::TemperatureToHumidity),
        "humidity-to-location map:" => Some(MappingType::HumidityToLocation),
        _ => None,
    }
}

fn parse_input(input: &str) -> (Vec<KeyRange>, HashMap<MappingType, Vec<MappingParameters>>) {
    let mut mapping_data: HashMap<MappingType, Vec<MappingParameters>> = HashMap::new();
    let mut seeds = Vec::new();

    let sections = input.split("\n\n");
    for (index, section) in sections.enumerate() {
        if index == 0 {
            seeds = parse_seeds(section);
            continue;
        }

        let maps = section.split('\n');
        let mut mapping_type: Option<MappingType> = None;
        for (index, map) in maps.enumerate() {
            if index == 0 {
                mapping_type = parse_mapping_type(map);
                continue;
            }

            let mut mapping_numbers = map.split(' ').map(|v| v.parse::<i64>().unwrap());

            let mapping_parameters = MappingParameters {
                destination_range_start: mapping_numbers.next().unwrap(),
                source_range_start: mapping_numbers.next().unwrap(),
                range_length: mapping_numbers.next().unwrap(),
            };

            mapping_data
                .entry(mapping_type.unwrap())
                .and_modify(|parameters| parameters.push(mapping_parameters))
                .or_insert(vec![mapping_parameters]);
        }
    }

    (seeds, mapping_data)
}

fn reverse_lookup(mapping_data: &Vec<MappingParameters>, value: i64) -> i64 {
    for mapping in mapping_data {
        let check = value - mapping.destination_range_start + mapping.source_range_start;
        if check < mapping.source_range_start + mapping.range_length
            && check >= mapping.source_range_start
        {
            return check;
        }
    }

    value
}

fn get_lowest_location(
    seed_ranges: Vec<KeyRange>,
    mapping_data: HashMap<MappingType, Vec<MappingParameters>>,
) -> i64 {
    let mut location = 46_i64;

    loop {
        let mut current = location;

        for mapping_type in MappingType::DEFAULT_MAPPING_ORDER.iter().rev() {
            current = reverse_lookup(mapping_data.get(mapping_type).unwrap(), current);
        }

        for seed_range in &seed_ranges {
            if current >= seed_range.start && current < seed_range.start + seed_range.length {
                return location;
            }
        }

        location += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("./input1.txt").trim();
        let (seeds, mapping_data) = parse_input(input);
        let output = get_lowest_location(seeds, mapping_data);
        assert_eq!(output, 46);
    }

    #[test]
    fn part1() {
        let input = include_str!("./input2.txt").trim();
        let (seeds, mapping_data) = parse_input(input);
        let output = get_lowest_location(seeds, mapping_data);
        assert_eq!(output, 20283860);
    }
}
