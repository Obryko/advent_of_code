use advent_of_code::Day;

#[derive(Debug, Copy, Clone)]
struct SectionRange(i32, i32);

impl SectionRange {
    pub fn new(value: String) -> Self {
        let values = value
            .split('-')
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        SectionRange(values[0], values[1])
    }

    pub fn contain(self, other: SectionRange) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn overlap(self, other: SectionRange) -> bool {
        self.1 >= other.0 && other.1 >= self.1 && self.0 <= other.1
    }
}

#[derive(Debug, Copy, Clone)]
struct Pair(SectionRange, SectionRange);

impl Pair {
    pub fn new(value: String) -> Self {
        let values = value.split(',').collect::<Vec<&str>>();
        Pair(
            SectionRange::new(values[0].to_string()),
            SectionRange::new(values[1].to_string()),
        )
    }

    pub fn contain(&self) -> bool {
        self.0.contain(self.1) || self.1.contain(self.0)
    }

    pub fn overlap(&self) -> bool {
        self.0.overlap(self.1) || self.1.overlap(self.0)
    }
}

#[derive(Default)]
pub struct Day4Of2022 {
    data: Vec<Pair>,
}

impl Day4Of2022 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Day for Day4Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 4)
    }

    fn parse(&mut self, data: String) {
        let res = data
            .split('\n')
            .filter(|pair| !pair.is_empty())
            .map(|pair| Pair::new(pair.to_string()))
            .collect();
        self.data = res;
    }

    fn task1(&mut self) -> String {
        self.data
            .iter()
            .filter(|&pair| pair.contain())
            .count()
            .to_string()
    }

    fn task2(&mut self) -> String {
        self.data
            .iter()
            .filter(|&pair| pair.overlap())
            .count()
            .to_string()
    }
}
