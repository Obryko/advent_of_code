use std::collections::HashSet;

use advent_of_code::Day;

#[derive(Default)]
pub struct Day6Of2022 {
    data: Vec<char>,
}

impl Day6Of2022 {
    fn find_marker(&self, size: usize) -> String {
        self.data
            .windows(size)
            .enumerate()
            .map(|(index, window)| (index, window.iter().collect::<HashSet<&char>>()))
            .find(|(_, window)| window.len().eq(&size))
            .map(|(index, _)| index + size)
            .unwrap_or(0)
            .to_string()
    }
}

impl Day for Day6Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 6)
    }

    fn parse(&mut self, data: String) {
        println!("----- Parsing data for a Day {} Year {}-----", self.get_day().1, self.get_day().0);
        self.data = data.split('\n').collect::<Vec<&str>>()[0].chars().collect();
    }

    fn task1(&self) -> String {
        self.find_marker(4)
    }
    fn task2(&self) -> String {
        self.find_marker(14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn task_1_1() {
        let day = Day6Of2022::new().parse(INPUT_1.to_string());
        assert_eq!(day.task1(), "7");
    }

    #[test]
    fn task_1_2() {
        let day = Day6Of2022::new().parse(INPUT_2.to_string());
        assert_eq!(day.task1(), "5");
    }

    #[test]
    fn task_1_3() {
        let day = Day6Of2022::new().parse(INPUT_3.to_string());
        assert_eq!(day.task1(), "6");
    }

    #[test]
    fn task_1_4() {
        let day = Day6Of2022::new().parse(INPUT_4.to_string());
        assert_eq!(day.task1(), "10");
    }

    #[test]
    fn task_1_5() {
        let day = Day6Of2022::new().parse(INPUT_5.to_string());
        assert_eq!(day.task1(), "11");
    }

    #[test]
    fn task_2_1() {
        let day = Day6Of2022::new().parse(INPUT_1.to_string());
        assert_eq!(day.task2(), "19");
    }

    #[test]
    fn task_2_2() {
        let day = Day6Of2022::new().parse(INPUT_2.to_string());
        assert_eq!(day.task2(), "23");
    }

    #[test]
    fn task_2_3() {
        let day = Day6Of2022::new().parse(INPUT_3.to_string());
        assert_eq!(day.task2(), "23");
    }

    #[test]
    fn task_2_4() {
        let day = Day6Of2022::new().parse(INPUT_4.to_string());
        assert_eq!(day.task2(), "29");
    }

    #[test]
    fn task_2_5() {
        let day = Day6Of2022::new().parse(INPUT_5.to_string());
        assert_eq!(day.task2(), "26");
    }
}
