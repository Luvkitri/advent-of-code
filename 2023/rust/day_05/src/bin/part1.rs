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

fn main() {
    let input = include_str!("./input2.txt").trim();
    let (seeds, mapping_data) = parse_input(input);
    let output = get_lowest_location(seeds, mapping_data);
    dbg!(output.unwrap());
}

fn parse_seeds(section: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(section)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect()
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

fn parse_input(input: &str) -> (Vec<u32>, HashMap<MappingType, Vec<MappingParameters>>) {
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

fn get_mapping(key: Option<i64>, mapping_parameters: &MappingParameters) -> Option<i64> {
    key.and_then(|key| {
        if key < mapping_parameters.source_range_start
            || key > mapping_parameters.source_range_start + mapping_parameters.range_length
        {
            return None;
        }

        Some(
            key - (mapping_parameters.source_range_start
                - mapping_parameters.destination_range_start),
        )
    })
}

fn get_lowest_location(
    seeds: Vec<u32>,
    mapping_data: HashMap<MappingType, Vec<MappingParameters>>,
) -> Option<i64> {
    let mut location = None;
    for seed in seeds {
        let mut key = Some(i64::from(seed));
        let mut mapped_value = None;
        for mapping_type in MappingType::DEFAULT_MAPPING_ORDER {
            for mapping_parameters in mapping_data.get(&mapping_type).unwrap() {
                mapped_value = get_mapping(key, mapping_parameters);

                if mapped_value.is_some() {
                    key = mapped_value;
                    break;
                }
            }
        }

        if location.is_none() || mapped_value.is_some_and(|v| location.is_some_and(|l| l > v)) {
            location = mapped_value;
        }

        if mapped_value.is_none() && key.is_some_and(|v| location.is_some_and(|l| l > v)) {
            location = key;
        }
    }

    location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("./input1.txt").trim();
        let (seeds, mapping_data) = parse_input(input);
        let output = get_lowest_location(seeds, mapping_data);
        assert_eq!(output.unwrap(), 35);
    }

    #[test]
    fn part1() {
        let input = include_str!("./input2.txt").trim();
        let (seeds, mapping_data) = parse_input(input);
        let output = get_lowest_location(seeds, mapping_data);
        assert_eq!(output.unwrap(), 313045984);
    }
}
