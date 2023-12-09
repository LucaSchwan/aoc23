use anyhow::Result;
use num::Integer;
use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn load_input(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn read_lines_of_num<T>(path: &str) -> Result<Vec<Vec<T>>>
where
    T: Integer + FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|value| value.parse::<T>().ok())
                .collect()
        })
        .collect())
}
