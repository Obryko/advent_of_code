use std::collections::HashMap;
use std::ops::Range;

use advent_of_code::Day;
use pariter::{IteratorExt, scope};

#[derive(Debug, Copy, Clone)]
struct CategoryMap {
    destination: i64,
    source: i64,
    range: i64,
}

impl CategoryMap {
    pub fn from_string(string: String) -> Self {
        let [destination_start, source_start, range]: [i64; 3] = string.trim().split(" ").map(|v| v.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap();
        Self::new(destination_start, source_start, range)
    }
    pub fn new(destination: i64, source: i64, range: i64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }

    fn get_max_source(&self) -> i64 {
        self.source + self.range - 1
    }

    pub fn get_destination_for_source(&self, source: i64) -> Option<i64> {
        if !(source.ge(&self.source) && source.le(&self.get_max_source())) {
            return None;
        }
        Some(self.destination + (source - self.source))
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
    pub fn new(categories: HashMap<&str, String>) -> Self {
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

    fn category_map_list_from_string(string: &String) -> Vec<CategoryMap> {
        string.split("\n").map(|v| CategoryMap::from_string(v.to_string())).collect()
    }

    pub fn get_location_for_seed(&self, seed: i64) -> i64 {
        let soil = get_destination_for_source(&self.seed_to_soil, seed);
        let fertilizer = get_destination_for_source(&self.soil_to_fertilizer, soil);
        let water = get_destination_for_source(&self.fertilizer_to_water, fertilizer);
        let light = get_destination_for_source(&self.water_to_light, water);
        let temperature = get_destination_for_source(&self.light_to_temperature, light);
        let humidity = get_destination_for_source(&self.temperature_to_humidity, temperature);
        get_destination_for_source(&self.humidity_to_location, humidity)
    }
}

fn get_destination_for_source(mapper: &Vec<CategoryMap>, source: i64) -> i64 {
    mapper.iter().find_map(|v| v.get_destination_for_source(source)).unwrap_or(source)
}

#[derive(Default, Debug)]
pub struct Day5Of2023 {
    seeds: Vec<i64>,
    garden: Garden,
}

impl Day for Day5Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 5)
    }

    fn parse(&mut self, data: String) {
        let (seeds, categories) = data.split_once("\n").unwrap();
        self.seeds = (seeds[7..]).split(" ").map(|v| v.parse().unwrap()).collect();
        let categories_mappers = categories
            .split("\n\n")
            .filter_map(|category| category.split_once("map:\n").map(|(name, mapper)| (name.trim(), mapper.trim().to_string())))
            .collect::<HashMap<&str, String>>();
        self.garden = Garden::new(categories_mappers);
    }

    fn task1(&self) -> String {
        self.seeds.iter()
            .map(|seed| self.garden.get_location_for_seed(*seed))
            .min().unwrap().to_string()
    }

    fn task2(&self) -> String {
        scope(|s| {
            (0..(self.seeds.len() / 2))
                .map(|i| i * 2)
                .map(|i| {
                    let start = *self.seeds.get(i).unwrap();
                    let range = *self.seeds.get(i + 1).unwrap();
                    start..(start + range - 1)
                })
                .parallel_map_scoped_custom(s, |o| o.threads(16), |seeds: Range<i64>| {
                    println!("Start seeds range {:?}", seeds);
                    let min = seeds.fold(i64::MAX,|acc,seed| {
                        let location = self.garden.get_location_for_seed(seed);
                        match location.le(&acc) {
                            true => location,
                            false => acc
                        }
                    });
                    println!("{:?}", min);
                    return min;
                })
                .min().unwrap().to_string()
        }).unwrap()
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
        let mut day = Day5Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "35");
    }

    #[test]
    fn task_2() {
        let mut day = Day5Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "46");
    }
}
