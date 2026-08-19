use dotenvy::var;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use reqwest::blocking::Client;
const INPUT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs");

fn read_day_input(year: usize, day: usize) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let uri = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session_id =
        var("SESSION_ID").map_err(|_| "SESSION_ID not set — copy .env.example to .env")?;
    let res = client
        .get(uri)
        .header("cookie", format!("session={session_id}"))
        .send()?
        .error_for_status()?;

    Ok(res.text()?)
}

pub fn get_day_input(year: usize, day: usize) -> Result<String, Box<dyn Error>> {
    println!("----- Load data for a Day {day} Year {year}-----");
    let path = PathBuf::from(format!("{INPUT_DIR}/{year}/{day}.txt"));

    if let Ok(cached) = fs::read_to_string(&path)
        && !cached.is_empty()
    {
        return Ok(cached);
    }

    let result = read_day_input(year, day)?;
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, &result)?;
    Ok(result)
}
