use crate::common::intervals::Interval;
use crate::Day;




#[derive(Default, Debug)]
struct Race {
    time: i64,
    record_distance: i64,
}

impl Race {
    fn new(time: i64, record_distance: i64) -> Self {
        Self { time, record_distance }
    }

    fn is_winning_hold_time(&self, hold_time: i64) -> bool {
        hold_time * (self.time - hold_time) > self.record_distance
    }

    fn possible_press_times(&self) -> Option<Interval> {
        let first = self.first_possible_press_time()?;
        Interval::new(first, self.time - first).ok()
    }

    fn first_possible_press_time(&self) -> Option<i64> {
        let mut low = 0;
        let mut high = self.time / 2;
        while low < high {
            let mid = low + (high - low) / 2;

            if self.is_winning_hold_time(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        self.is_winning_hold_time(low).then_some(low)
    }

    fn possible_press_times_count(&self) -> i64 {
        self.possible_press_times()
            .map(|interval| interval.len())
            .unwrap_or(0)
    }
}

#[derive(Default, Debug)]
pub struct Day6Of2023 {
    races: Vec<Race>,
    race: Race,
}

impl Day6Of2023 {
    fn parse_line(line: &str) -> Vec<i64> {
        line.split_once(':')
            .map(|(_, values)| values
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect())
            .expect("Invalid input")
    }

    fn parse_for_races(&mut self, times: &str, distances: &str) {
        let parse_times = Self::parse_line(times);
        let parse_distances = Self::parse_line(distances);
        assert_eq!(
            parse_times.len(),
            parse_distances.len(),
            "Not same amount of values."
        );

        self.races = parse_times.into_iter().zip(parse_distances)
            .map(|(t, d)| Race::new(t, d))
            .collect();
    }

    fn parse_joined_line(line: &str) -> i64 {
        line.split_once(':')
            .map(|(_, values)| {
                values
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            })
            .expect("Invalid input")
    }

    fn parse_for_race(&mut self, times: &str, distances: &str) {
        self.race = Race::new(Self::parse_joined_line(times), Self::parse_joined_line(distances));
    }
}

impl Day for Day6Of2023 {

    fn parse(&mut self, data: String) {
        let (times, distances) = data.split_once('\n').expect("Invalid input");

        self.parse_for_races(times, distances);
        self.parse_for_race(times, distances);
    }

    fn task1(&self) -> String {
        self.races.iter()
            .map(|r| r.possible_press_times_count())
            .product::<i64>()
            .to_string()
    }

    fn task2(&self) -> String {
        self.race.possible_press_times().map(|i| i.len()).unwrap_or(0).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn task_1() {
        let mut day = Day6Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "288");
    }

    #[test]
    fn task_2() {
        let mut day = Day6Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "71503");
    }

    #[test]
    fn should_find_single_possible_press_time() {
        let race = Race::new(2, 0);

        assert_eq!(
            race.possible_press_times(),
            Some(Interval::new(1, 1).unwrap())
        );
    }

    #[test]
    fn should_find_first_possible_press_time() {
        let race = Race::new(7, 9);

        assert_eq!(race.first_possible_press_time(), Some(2));
    }

    #[test]
    fn should_return_none_when_race_cannot_be_won() {
        let race = Race::new(7, 1000);

        assert_eq!(race.first_possible_press_time(), None);
    }
}
