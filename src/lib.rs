use anyhow::Result;
use num::Integer;
use std::{
    self,
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

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

pub fn read_lines_of_num<T>(path: &str, delim: &str) -> Result<Vec<Vec<T>>>
where
    T: Integer + FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            line.split(delim)
                .filter_map(|value| value.parse::<T>().ok())
                .collect()
        })
        .collect())
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Vec2D {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const UP: Self = Self { x: 0, y: -1 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
