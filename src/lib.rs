use async_trait::async_trait;

#[async_trait]
pub trait Day {
    async fn init(&mut self) -> () {
        let (year, day) = self.get_day();
        let data: String = inputs::get_day_input(year, day).await;
        self.parse(data);
    }
    fn get_day(&self) -> (i32, i32);
    fn parse(&mut self, data: String);
    fn task1(&mut self) -> String;
    fn task2(&mut self) -> String;
}

pub async fn run_day<T: Day + ?Sized + Send>(day: &mut T) {
    println!("----- Paring Day -----");
    day.init().await;
    println!("----- Task 1 -----");
    let task1 = day.task1();
    println!("Task 1: {}", task1);
    println!("----- Task 2 -----");
    let task2 = day.task2();
    println!("Task 2: {}", task2);
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

    pub async fn get_day_input(year: i32, day: i32) -> String {
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
