use aoc_toolkit::core::Registry;

mod aoc2015;
mod aoc2022;
mod aoc2023;

pub static REGISTRY: Registry = Registry::new(&[aoc2015::DAYS, aoc2022::DAYS, aoc2023::DAYS]);
