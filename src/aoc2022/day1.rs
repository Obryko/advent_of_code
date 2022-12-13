use advent_of_code::Day;

#[derive(Default)]
pub struct Day1Of2022 {
    data: Vec<i32>,
}

impl Day1Of2022 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Day for Day1Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 1)
    }

    fn parse(&mut self, data: String) {
        let res: Vec<i32> = data
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|calories| calories.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .collect();
        self.data = res;
    }

    fn task1(&mut self) -> String {
        self.data.iter().max().unwrap_or(&0).to_string()
    }
    fn task2(&mut self) -> String {
        self.data.sort();
        self.data.iter().rev().take(3).sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
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
