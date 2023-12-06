use advent_of_code::Day;


const VELOCITY_DELTA: i64 = 1;

#[derive(Default, Debug)]
struct Race(i64, i64);


fn calculate_distance(time: i64, velocity: i64) -> i64 {
    time * velocity
}

fn calculate_velocity_for_pressed_button(time: i64, delta: i64) -> i64 {
    time * delta
}

impl Race {

    fn is_possible_in_time(&self, time: i64) -> bool {
        calculate_distance(self.0 - time, calculate_velocity_for_pressed_button(time, VELOCITY_DELTA)) > self.1
    }

    fn get_possible_times_for_press_button(&self) -> Vec<i64> {
        let mut times = 0..=self.0;

        let first = times.find(|&t| self.is_possible_in_time(t)).unwrap();
        let last = times.rfind(|&t| self.is_possible_in_time(t)).unwrap();

        (first..=last).collect::<Vec<i64>>()
    }
}

#[derive(Default, Debug)]
pub struct Day6Of2023 {
    races: Vec<Race>,
    race: Race,
}

fn parse_line(line: String) -> Vec<i64> {
    line.split_once(":").map(|(_, values)| values.trim().split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>()).expect("Invalid input")
}

impl Day6Of2023 {
    fn parse_for_races(&mut self, times: &str, distances: &str) {
        let parse_times = parse_line(times.to_string());
        let parse_distances = parse_line(distances.to_string());
        if !parse_times.len().eq(&parse_distances.len()) { panic!("Not same amount of values.") }

        self.races = parse_times.iter().zip(parse_distances.iter())
            .map(|(t, d)| Race(*t, *d))
            .collect::<Vec<Race>>();
    }

    fn parse_for_race(&mut self, times: &str, distances: &str) {
        let parse_times = parse_line(times.replace(" ", "")).first().unwrap().to_owned();
        let parse_distances = parse_line(distances.replace(" ", "")).first().unwrap().to_owned();

        self.race = Race(parse_times, parse_distances);
    }
}

impl Day for Day6Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 6)
    }

    fn parse(&mut self, data: String) {
        let (times, distances) = data.split_once("\n").expect("Invalid input");

        self.parse_for_races(times, distances);
        self.parse_for_race(times, distances);
    }

    fn task1(&self) -> String {
        self.races.iter()
            .map(|r| r.get_possible_times_for_press_button().len())
            .fold(1, |acc, curr| acc * curr)
            .to_string()
    }

    fn task2(&self) -> String {
        self.race.get_possible_times_for_press_button().len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn task_1() {
        let mut day = Day6Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "288");
    }

    #[test]
    fn task_2() {
        let mut day = Day6Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "71503");
    }
}
