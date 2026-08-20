use std::collections::HashMap;

use crate::Day;
use crate::common::intervals::Interval;

#[derive(Debug, Copy, Clone)]
struct CategoryMap {
    source: Interval,
    offset: i64
}

impl CategoryMap {
    fn from_string(string: &str) -> Self {
        let [destination_start, source_start, length]: [i64; 3] = string
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self::new(destination_start, source_start, length)
    }
    fn new(destination: i64, source: i64, length: i64) -> Self {
        Self {
            offset: destination - source,
            source: Interval::new(source, source + length - 1).unwrap(),
        }
    }

    fn map_value(&self, source: i64) -> Option<i64> {
        self.source.contains(source).then_some(self.offset + source)
    }
}

#[derive(Default, Debug)]
struct Garden {
    seed_to_soil: Vec<CategoryMap>,
    soil_to_fertilizer: Vec<CategoryMap>,
    fertilizer_to_water: Vec<CategoryMap>,
    water_to_light: Vec<CategoryMap>,
    light_to_temperature: Vec<CategoryMap>,
    temperature_to_humidity: Vec<CategoryMap>,
    humidity_to_location: Vec<CategoryMap>,
}


impl Garden {
    fn new(categories: HashMap<&str, String>) -> Self {
        Self {
            seed_to_soil: Garden::category_map_list_from_string(categories.get("seed-to-soil").unwrap()),
            soil_to_fertilizer: Garden::category_map_list_from_string(categories.get("soil-to-fertilizer").unwrap()),
            fertilizer_to_water: Garden::category_map_list_from_string(categories.get("fertilizer-to-water").unwrap()),
            water_to_light: Garden::category_map_list_from_string(categories.get("water-to-light").unwrap()),
            light_to_temperature: Garden::category_map_list_from_string(categories.get("light-to-temperature").unwrap()),
            temperature_to_humidity: Garden::category_map_list_from_string(categories.get("temperature-to-humidity").unwrap()),
            humidity_to_location: Garden::category_map_list_from_string(categories.get("humidity-to-location").unwrap()),
        }
    }

    fn categories(&self) -> [&[CategoryMap]; 7] {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
    }

    fn category_map_list_from_string(string: &str) -> Vec<CategoryMap> {
        string.lines().map(CategoryMap::from_string).collect()
    }

    fn map_value(mapper: &[CategoryMap], source: i64) -> i64 {
        mapper.iter().find_map(|v| v.map_value(source)).unwrap_or(source)
    }

    fn location_for_seed(&self, seed: i64) -> i64 {
        self.categories()
            .into_iter()
            .fold(seed, |value, category| Self::map_value(category, value))
    }

    fn locations_for_seed_intervals(
        &self,
        intervals: Vec<Interval>,
    ) -> Vec<Interval> {
        self.categories()
            .into_iter()
            .fold(intervals, |value, category| Self::map_intervals(category, value))
    }

    fn map_interval(
        category_maps: &[CategoryMap],
        interval: Interval,
    ) -> Vec<Interval> {
        let mut mapped: Vec<Interval> = vec![];
        let mut remaining = vec![interval];

        for category_map in category_maps {
            let mut next_remaining = vec![];
            while let Some(interval) = remaining.pop() {
                if let Some(overlap) = category_map.source.intersection(&interval) {
                    mapped.push(overlap.shift(category_map.offset));
                    next_remaining.extend(interval - overlap);
                } else {
                    next_remaining.push(interval);
                }
            }
            remaining = next_remaining;
        }
        mapped.extend(remaining);
        mapped
    }

    fn map_intervals(
        category_maps: &[CategoryMap],
        intervals: Vec<Interval>,
    ) -> Vec<Interval> {
        intervals.into_iter()
            .flat_map(|interval| Self::map_interval(category_maps, interval))
            .collect()
    }
}


#[derive(Default, Debug)]
pub struct Day5Of2023 {
    seeds: Vec<i64>,
    garden: Garden,
}

impl Day for Day5Of2023 {
    fn parse(&mut self, data: String) {
        let (seeds, categories) = data.split_once('\n').unwrap();
        self.seeds = (seeds[7..]).split_whitespace().map(|v| v.parse().unwrap()).collect();
        let categories_mappers = categories
            .split("\n\n")
            .filter_map(|category| category.split_once("map:\n").map(|(name, mapper)| (name.trim(), mapper.trim().to_string())))
            .collect::<HashMap<&str, String>>();
        self.garden = Garden::new(categories_mappers);
    }

    fn task1(&self) -> String {
        self.seeds.iter()
            .copied()
            .map(|seed| self.garden.location_for_seed(seed))
            .min().unwrap().to_string()
    }

    fn task2(&self) -> String {
        let seeds: Vec<Interval> = self.seeds
            .chunks_exact(2)
            .map(|pair| {
                let start = pair[0];
                let length = pair[1];
                (start, start + length - 1).try_into().unwrap()
            }).collect();
        self.garden.locations_for_seed_intervals(seeds)
            .into_iter()
            .map(|interval| interval.start())
            .min().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

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
                        56 93 4";

    #[test]
    fn task_1() {
        let mut day = Day5Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "35");
    }

    #[test]
    fn task_2() {
        let mut day = Day5Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "46");
    }

    #[test]
    fn should_map_source_to_destination() {
        let map = CategoryMap::new(50, 98, 2);

        assert_eq!(map.map_value(98), Some(50));
        assert_eq!(map.map_value(99), Some(51));
        assert_eq!(map.map_value(100), None);
    }

    #[test]
    fn should_map_intervals_for_garden() {
        let categories_maps = vec![
            CategoryMap::new(110, 10, 5),
            CategoryMap::new(218, 18, 3)
        ];

        let input = Interval::new(8, 22).unwrap();

        let mut result = Garden::map_interval(&categories_maps, input);
        result.sort_by_key(|interval| interval.start());

        assert_eq!(
            result,
            vec![
                Interval::new(8, 9).unwrap(),
                Interval::new(15, 17).unwrap(),
                Interval::new(21, 22).unwrap(),
                Interval::new(110, 114).unwrap(),
                Interval::new(218, 220).unwrap(),
            ]
        );
    }

    #[test]
    fn should_map_multiple_intervals_for_garden() {
        let category_maps = vec![
            CategoryMap::new(110, 10, 5),
        ];

        let intervals = vec![
            Interval::new(8, 12).unwrap(),
            Interval::new(20, 22).unwrap(),
        ];

        let mut result = Garden::map_intervals(&category_maps, intervals);
        result.sort_by_key(|interval| interval.start());

        assert_eq!(
            result,
            vec![
                Interval::new(8, 9).unwrap(),
                Interval::new(20, 22).unwrap(),
                Interval::new(110, 112).unwrap(),
            ]
        );
    }

    #[test]
    fn should_not_map_already_mapped_interval_twice_in_same_category() {
        let category_maps = vec![
            CategoryMap::new(20, 10, 3), // [10..12] -> [20..22]
            CategoryMap::new(30, 20, 3), // [20..22] -> [30..32]
        ];

        let input = Interval::new(10, 12).unwrap();

        let result = Garden::map_interval(&category_maps, input);

        assert_eq!(
            result,
            vec![Interval::new(20, 22).unwrap()]
        );
    }
}
