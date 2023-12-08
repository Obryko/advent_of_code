use std::collections::HashMap;

use advent_of_code::Day;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node(String);

impl Node {
    fn ends_with(&self, c: char) -> bool {
        self.0.ends_with(c)
    }
}

const RIGHT: char = 'R';
const LEFT: char = 'L';

#[derive(Debug)]
struct Either<T>(T, T);

impl<T> Either<T> where T: Clone {
    fn new(left: T, right: T) -> Self {
        Self(left, right)
    }
    fn left(&self) -> T {
        return self.0.clone();
    }
    fn right(&self) -> T {
        return self.1.clone();
    }
    fn get(&self, dir: char) -> T {
        match dir {
            RIGHT => self.right(),
            LEFT => self.left(),
            _ => panic!("Invalid direction")
        }
    }
}

#[derive(Default, Debug)]
pub struct Day8Of2023 {
    steps: Vec<char>,
    map: HashMap<Node, Either<Node>>,
}

impl Day8Of2023 {
    fn find_end_node_value<'a, F: Fn(&Node) -> bool + 'a>(&self, start_node: Node, is_end_node: F) -> usize {
        let mut current_node = start_node;
        let mut counter: usize = 0;
        for &step in self.steps.iter().cycle() {
            current_node = self.map.get(&current_node).map(|e| e.get(step)).unwrap();
            counter += 1;
            if is_end_node(&current_node) { break; }
        }
        counter
    }
}

const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

const fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

impl Day for Day8Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 8)
    }

    fn parse(&mut self, data: String) {
        let (steps, map) = data.split_once("\n\n").unwrap();
        self.steps = steps.chars().collect();

        self.map = map.lines().map(|line| {
            let (result, either) = line.split_once(" = ")
                .map(|(r, e)| (r.to_string(), e.replace("(", "").replace(")", "").to_string()))
                .unwrap();

            let (left, right) = either.trim().split_once(", ").unwrap();
            (Node(result.to_string()), Either::new(Node(left.to_string()), Node(right.to_string())))
        }).collect::<HashMap<Node, Either<Node>>>();
    }

    fn task1(&self) -> String {
        let start_node = Node("AAA".to_string());
        let end_node = &Node("ZZZ".to_string());
        self.find_end_node_value(start_node, |n| n.eq(&end_node)).to_string()
    }

    fn task2(&self) -> String {
        let is_end_node = |n: &Node| n.ends_with('Z');
        self.map.keys()
            .filter(|&n| n.ends_with('A'))
            .map(|n| self.find_end_node_value(n.clone(), is_end_node))
            .fold(1, lcm)
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    const INPUT_1: &str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    const INPUT_2: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";

    #[test]
    fn task_1_1() {
        let mut day = Day8Of2023::new();
        day.parse(INPUT_1.to_string());
        assert_eq!(day.task1(), "2");
    }

    #[test]
    fn task_1_2() {
        let mut day = Day8Of2023::new();
        day.parse(INPUT_2.to_string());
        assert_eq!(day.task1(), "6");
    }

    const INPUT_3: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    #[test]
    fn task_2() {
        let mut day = Day8Of2023::new();
        day.parse(INPUT_3.to_string());

        let start = Instant::now();
        let res = day.task2();
        let duration = start.elapsed();

        println!("Time is: {:?}", duration);
        assert_eq!(res, "6");
    }
}
