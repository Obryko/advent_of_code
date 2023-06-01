use advent_of_code::Day;

#[derive(Default)]
struct Dir<'a> {
    name: String,
    files: Vec<(String, i32)>,
    dirs: Vec<Dir<'a>>,
    parent: Option<&'a Dir<'a>>,
}

impl<'a> Dir<'a> {
    pub fn new(name: String, parent: Option<&'a Dir<'a>>) -> Self {
        Self {
            name,
            parent,
            ..Self::default()
        }
    }

    fn add_file(&mut self, name: String, size: i32) {
        self.files.push((name, size));
    }

    fn add_dir(&mut self, name: String) {
        self.dirs.push(Dir::new(name, Some(self)));
    }

    fn get_size(&self) -> i32 {
        self.files.iter().map(|(_, size)| size).sum::<i32>()
            + self.dirs.iter().map(|dir| dir.get_size()).sum::<i32>()
    }
}

#[derive(Default)]
pub struct Day7Of2022 {
    data: Vec<String>,
}

impl Day7Of2022 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Day for Day7Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 7)
    }

    fn parse(&mut self, data: String) {
        // self.data = data.split('\n').collect::<Vec<String>>();
    }

    fn task1(&mut self) -> String {
        todo!();
    }
    fn task2(&mut self) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn task_1() {
        let mut day = Day7Of2022::new();
        day.parse(INPUT.to_string());

        assert_eq!(day.task1(), "95437");
    }

    #[test]
    fn task_2() {
        let mut day = Day7Of2022::new();
        day.parse(INPUT.to_string());

        assert_eq!(day.task2(), "");
    }
}
