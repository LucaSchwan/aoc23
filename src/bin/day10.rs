use anyhow::{Error, Result};
use aoc23::Vec2D;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    Top2Right,
    Top2Left,
    Bottom2Left,
    Bottom2Right,
    Ground,
    StartingPosition,
}

impl Pipe {
    fn next(&self, direction: Direction) -> (Vec2D, Direction) {
        use Direction::*;
        use Pipe::*;

        match direction {
            Up => match self {
                Vertical => (Vec2D::UP, Up),
                Bottom2Left => (Vec2D::LEFT, Left),
                Bottom2Right => (Vec2D::RIGHT, Right),
                _ => panic!("Impossible direction: {:?} {:?}", direction, self),
            },
            Right => match self {
                Horizontal => (Vec2D::RIGHT, Right),
                Top2Left => (Vec2D::UP, Up),
                Bottom2Left => (Vec2D::DOWN, Down),
                _ => panic!("Impossible direction: {:?} {:?}", direction, self),
            },
            Down => match self {
                Vertical => (Vec2D::DOWN, Down),
                Top2Right => (Vec2D::RIGHT, Right),
                Top2Left => (Vec2D::LEFT, Left),
                _ => panic!("Impossible direction: {:?} {:?}", direction, self),
            },
            Left => match self {
                Horizontal => (Vec2D::LEFT, Left),
                Top2Right => (Vec2D::UP, Up),
                Bottom2Right => (Vec2D::DOWN, Down),
                _ => panic!("Impossible direction: {:?} {:?}", direction, self),
            },
        }
    }
}

impl FromStr for Pipe {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        use Pipe::*;

        match s {
            "|" => Ok(Vertical),
            "-" => Ok(Horizontal),
            "L" => Ok(Top2Right),
            "J" => Ok(Top2Left),
            "7" => Ok(Bottom2Left),
            "F" => Ok(Bottom2Right),
            "." => Ok(Ground),
            "S" => Ok(StartingPosition),
            _ => Err(Error::msg("Couldn't parse field")),
        }
    }
}

type Field = BTreeMap<Vec2D, Pipe>;

fn parse_field(input: Vec<String>) -> (Vec2D, Field) {
    input.into_iter().enumerate().fold(
        (Vec2D::ZERO, BTreeMap::new()),
        |(mut starting_position, mut field), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let pipe = c.to_string().parse().expect("Should be a valid Pipe");
                let pos = Vec2D::new((x + 1) as i32, (y + 1) as i32);

                if pipe == Pipe::StartingPosition {
                    starting_position = pos;
                }

                field.insert(pos, pipe);
            });
            (starting_position, field)
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn starting_directions(
    starting_position: Vec2D,
    field: &Field,
) -> ((Vec2D, Direction), (Vec2D, Direction)) {
    use Direction::*;
    use Pipe::*;

    let mut directions = vec![];

    let direction = starting_position + Vec2D::new(0, 1);
    if let Some(pipe) = field.get(&direction) {
        match pipe {
            Vertical | Top2Right | Top2Left => directions.push((direction, Down)),
            _ => (),
        }
    }

    let direction = starting_position + Vec2D::new(0, -1);
    if let Some(pipe) = field.get(&direction) {
        match pipe {
            Vertical | Bottom2Right | Bottom2Left => directions.push((direction, Up)),
            _ => (),
        }
    }

    let direction = starting_position + Vec2D::new(1, 0);
    if let Some(pipe) = field.get(&direction) {
        match pipe {
            Horizontal | Top2Left | Bottom2Left => directions.push((direction, Right)),
            _ => (),
        }
    }

    let direction = starting_position + Vec2D::new(-1, 0);
    if let Some(pipe) = field.get(&direction) {
        match pipe {
            Horizontal | Top2Right | Bottom2Right => directions.push((direction, Left)),
            _ => (),
        }
    }

    debug_assert!(directions.len() == 2);

    (directions[0], directions[1])
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let (starting_position, field) = parse_field(input);

    let ((mut left_position, mut left_direction), (mut right_position, mut right_direction)) =
        starting_directions(starting_position, &field);

    let mut distance = 1;

    while left_position != right_position {
        let left_pipe = field.get(&left_position).expect("Should exist");
        let left_next = left_pipe.next(left_direction);
        left_position = left_position + left_next.0;
        left_direction = left_next.1;

        let right_pipe = field.get(&right_position).expect("Should exist");
        let right_next = right_pipe.next(right_direction);
        right_position = right_position + right_next.0;
        right_direction = right_next.1;

        distance += 1;
    }

    Ok(distance)
}

fn part2(path: &str) -> Result<u32> {
    let input = aoc23::read_one_per_line::<String>(path)?;
    let (starting_position, field) = parse_field(input.clone());

    let ((pre_starting_position, _), (mut position, mut direction)) =
        starting_directions(starting_position, &field);

    let mut pipe_loop = BTreeMap::new();

    match (
        starting_position - position,
        pre_starting_position - starting_position,
    ) {
        (Vec2D::DOWN, Vec2D::UP) | (Vec2D::UP, Vec2D::DOWN) => {
            pipe_loop.insert(starting_position, &Pipe::Vertical)
        }
        (Vec2D::DOWN, Vec2D::LEFT) | (Vec2D::LEFT, Vec2D::DOWN) => {
            pipe_loop.insert(starting_position, &Pipe::Bottom2Right)
        }
        (Vec2D::DOWN, Vec2D::RIGHT) | (Vec2D::RIGHT, Vec2D::DOWN) => {
            pipe_loop.insert(starting_position, &Pipe::Bottom2Left)
        }
        (Vec2D::UP, Vec2D::RIGHT) | (Vec2D::RIGHT, Vec2D::UP) => {
            pipe_loop.insert(starting_position, &Pipe::Top2Left)
        }
        (Vec2D::UP, Vec2D::LEFT) | (Vec2D::LEFT, Vec2D::UP) => {
            pipe_loop.insert(starting_position, &Pipe::Top2Right)
        }
        (Vec2D::RIGHT, Vec2D::LEFT) | (Vec2D::LEFT, Vec2D::RIGHT) => {
            pipe_loop.insert(starting_position, &Pipe::Horizontal)
        }
        _ => panic!("Shouldn't happen"),
    };

    while position != starting_position {
        let pipe = field.get(&position).expect("Should exist");
        let next = pipe.next(direction);

        pipe_loop.insert(position, pipe);

        direction = next.1;
        position = position + next.0;
    }

    let mut inside = false;
    let mut wall_beginning = Pipe::Ground;
    let mut inside_count = 0;

    (1..(input.len() + 1)).for_each(|y| {
        (1..(input[0].len() + 1)).for_each(|x| {
            match pipe_loop.get(&Vec2D::new(x as i32, y as i32)) {
                Some(pipe) => match pipe {
                    Pipe::Vertical => inside = !inside,
                    Pipe::Top2Right => wall_beginning = Pipe::Top2Right,
                    Pipe::Top2Left => {
                        if let Pipe::Bottom2Right = wall_beginning {
                            inside = !inside;
                        }
                    }
                    Pipe::Bottom2Right => wall_beginning = Pipe::Bottom2Right,
                    Pipe::Bottom2Left => {
                        if let Pipe::Top2Right = wall_beginning {
                            inside = !inside;
                        }
                    }
                    _ => {}
                },
                None => {
                    if inside {
                        inside_count += 1;
                    }
                }
            }
        });
    });

    Ok(inside_count)
}

fn _print_input(input: Vec<String>, pipe_loop: &BTreeMap<Vec2D, &Pipe>) {
    (1..(input.len() + 1)).for_each(|y| {
        (1..(input[0].len() + 1)).for_each(|x| {
            match pipe_loop.get(&Vec2D::new(x as i32, y as i32)) {
                Some(pipe) => match pipe {
                    Pipe::Vertical => print!("│"),
                    Pipe::Horizontal => print!("─"),
                    Pipe::Top2Right => print!("└"),
                    Pipe::Top2Left => print!("┘"),
                    Pipe::Bottom2Left => print!("┐"),
                    Pipe::Bottom2Right => print!("┌"),
                    Pipe::Ground => print!("0"),
                    Pipe::StartingPosition => print!("S"),
                },
                None => print!("\u{2022}"),
            }
        });
        println!();
    });
}

fn main() {
    println!("Part 1: {}", part1("data/10.input").unwrap());
    println!("Part 2: {}", part2("data/10.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("data/10_1.sample", 4 ; "Simple loop")]
    #[test_case("data/10_2.sample", 8 ; "More complex loop")]
    fn part1_test(path: &str, result: u32) {
        assert_eq!(part1(path).unwrap(), result);
    }

    #[test_case("data/10_3.sample", 4 ; "Simple loop")]
    #[test_case("data/10_4.sample", 4 ; "Simple loop with narrow passage")]
    #[test_case("data/10_5.sample", 8 ; "Larger example")]
    #[test_case("data/10_6.sample", 10 ; "Extra larger example")]
    fn part2_test(path: &str, result: u32) {
        assert_eq!(part2(path).unwrap(), result);
    }
}
