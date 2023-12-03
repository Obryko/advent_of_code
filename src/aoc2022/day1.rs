use advent_of_code::Day;

#[derive(Default)]
pub struct Day1Of2022 {
    data: Vec<i32>,
}


impl Day for Day1Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 1)
    }

    fn parse(&mut self, data: String) {
        println!("----- Parsing data for a Day {} Year {}-----", self.get_day().1, self.get_day().0);
        self.data = data
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|calories| calories.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .collect();
    }

    fn task1(&self) -> String {
        self.data.iter().max().unwrap_or(&0).to_string()
    }
    fn task2(&self) -> String {
        let mut data = self.data.clone();
        data.sort();
        data.iter().rev().take(3).sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn task_1() {
        let mut day = Day1Of2022::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "24000");
    }

    #[test]
    fn task_2() {
        let mut day = Day1Of2022::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "45000");
    }
}
