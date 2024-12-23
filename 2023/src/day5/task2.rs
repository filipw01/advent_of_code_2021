use itertools::Itertools;
use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let converter = Converter::from(input);
    converter
        .seeds
        .iter()
        .map(|seed| converter.calculate_location(seed))
        .min()
        .unwrap()
}

struct ConverterRange {
    length: usize,
    source_start: usize,
    destination_start: usize,
}

impl From<&str> for ConverterRange {
    fn from(value: &str) -> Self {
        let (dest, src, length) = value.split(' ').collect_tuple().unwrap();
        Self {
            destination_start: dest.parse().unwrap(),
            source_start: src.parse().unwrap(),
            length: length.parse().unwrap(),
        }
    }
}

struct Converter {
    seeds: HashSet<usize>,
    seed_to_soil: Vec<ConverterRange>,
    soil_to_fertilizer: Vec<ConverterRange>,
    fertilizer_to_water: Vec<ConverterRange>,
    water_to_light: Vec<ConverterRange>,
    light_to_temperature: Vec<ConverterRange>,
    temperature_to_humidity: Vec<ConverterRange>,
    humidity_to_location: Vec<ConverterRange>,
}

impl Converter {
    fn calculate(source: &usize, ranges: &Vec<ConverterRange>) -> usize {
        if let Some(range) = ranges
            .iter()
            .find(|range| (range.source_start..range.source_start + range.length).contains(source))
        {
            return source + range.destination_start - range.source_start;
        }
        *source
    }
    fn calculate_location(&self, seed: &usize) -> usize {
        let soil = Converter::calculate(seed, &self.seed_to_soil);
        let fertilizer = Converter::calculate(&soil, &self.soil_to_fertilizer);
        let water = Converter::calculate(&fertilizer, &self.fertilizer_to_water);
        let light = Converter::calculate(&water, &self.water_to_light);
        let temperature = Converter::calculate(&light, &self.light_to_temperature);
        let humidity = Converter::calculate(&temperature, &self.temperature_to_humidity);
        Converter::calculate(&humidity, &self.humidity_to_location)
    }
}

impl From<&str> for Converter {
    fn from(value: &str) -> Self {
        let mut segments = value.split("\n\n");
        let seeds: HashSet<usize> = segments
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split_whitespace()
            .map(|seed| seed.parse().unwrap())
            .collect();

        let (
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ) = segments
            .map(|segment| {
                segment
                    .split_once('\n')
                    .unwrap()
                    .1
                    .lines()
                    .map(ConverterRange::from)
                    .collect()
            })
            .collect_tuple()
            .unwrap();

        Self {
            seeds,
            fertilizer_to_water,
            humidity_to_location,
            light_to_temperature,
            seed_to_soil,
            soil_to_fertilizer,
            temperature_to_humidity,
            water_to_light,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
            ),
            35
        );
    }
}
