use crate::{RegisteredDay, aoc2015, aoc2022, aoc2023};
use std::collections::BTreeMap;

type YearLoader = fn() -> BTreeMap<usize, RegisteredDay>;

pub const YEARS: &[(usize, YearLoader)] = &[
    (2015, aoc2015::get_year_days),
    (2022, aoc2022::get_year_days),
    (2023, aoc2023::get_year_days),
];
