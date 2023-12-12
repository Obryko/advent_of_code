macro_rules! run_task {
    ($self:ident, $F:ident, $n:literal) => {
        println!("----- Task {} -----", $n);
        let time = std::time::Instant::now();
        let task = $self.$F();
        println!("Task {}: {}, time: {:?}", $n, task, time.elapsed());
    };
}

pub enum Part {
    FIRST,
    SECOND,
}

pub trait Day {
    fn new() -> Self where Self: Default {
        Self::default()
    }

    fn get_day(&self) -> (i32, i32);
    fn parse(&mut self, data: String);
    fn task1(&self) -> String;
    fn task2(&self) -> String;
    fn run(&self, part: Option<Part>) -> () {
        match part {
            Some(Part::FIRST) => { run_task!(self, task1, 1); }
            Some(Part::SECOND) => { run_task!(self, task2, 2); }
            None => {
                run_task!(self, task1, 1);
                run_task!(self, task2, 2);
            }
        };
    }
}

pub mod inputs {
    use std::fs::{create_dir_all, File, OpenOptions};
    use std::io::{Read, Write};
    use std::path::Path;

    use reqwest::Client;

    fn get_day_file_or_create_if_not_exist(year: i32, day: i32) -> File {
        let path = &format!("inputs/{}", year);
        create_dir_all(Path::new(path)).expect("Failed to create year inputs directories.");
        let file_path_string = &format!("{path}/{day}.txt");
        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(Path::new(file_path_string))
            .expect("Failed to create day file.")
    }

    async fn read_day_input(year: i32, day: i32) -> String {
        let client = Client::new();
        let uri = format!("https://adventofcode.com/{year}/day/{day}/input");
        let session_id = dotenv::var("SESSION_ID").unwrap();
        let res = client
            .get(uri)
            .header("cookie", format!("session={session_id}"))
            .send()
            .await
            .unwrap_or_else(|err| panic!("Error while attempting to retrieve input data for {year} task of day {day}. Error: {err}"));
        if res.status() != 200 {
            panic!("Cannot retrieve input data for {year} task of day {day}. Status: {}", res.status());
        }

        res.text().await.expect("Cannot parse body.")
    }

    pub async fn get_day_input((year, day): (i32, i32)) -> String {
        println!("----- Load data for a Day {} Year {}-----", day, year);
        let mut file = get_day_file_or_create_if_not_exist(year, day);
        let mut result = String::new();
        file.read_to_string(&mut result).unwrap_or_else(|err| {
            panic!("Cannot read a file for {year} task of day {day}. Error: {err}")
        });

        if result.is_empty() {
            result = read_day_input(year, day).await;
            file.write_all(result.as_bytes()).unwrap_or_else(|err| {
                panic!("Something goes wrong when try save content to file of {year} day {day}. Error: {err}")
            });
        }

        result
    }
}
