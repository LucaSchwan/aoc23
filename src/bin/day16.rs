use std::collections::{BTreeMap, HashSet};

use anyhow::Result;
use aoc23::Vec2D;
use itertools::Itertools;

#[derive(Clone)]
enum Field {
    Empty,
    RightDownMirror,
    RightUpMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

type Contraption = BTreeMap<Vec2D, Field>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Beam {
    head: Vec2D,
    direction: Direction,
}

impl Beam {
    fn start() -> Self {
        Self {
            head: Vec2D::new(0, 0),
            direction: Direction::Right,
        }
    }

    fn new(head: Vec2D, direction: Direction) -> Self {
        Self { head, direction }
    }

    fn next(&mut self) {
        self.head = self.head
            + match self.direction {
                Direction::Up => Vec2D::UP,
                Direction::Right => Vec2D::RIGHT,
                Direction::Down => Vec2D::DOWN,
                Direction::Left => Vec2D::LEFT,
            }
    }

    fn rotate(&mut self, field: &Field) {
        use Direction::*;
        self.direction = match field {
            Field::RightDownMirror => match self.direction {
                Up => Left,
                Right => Down,
                Down => Right,
                Left => Up,
            },
            Field::RightUpMirror => match self.direction {
                Up => Right,
                Right => Up,
                Down => Left,
                Left => Down,
            },
            _ => panic!("Shouldn't be passed"),
        }
    }
}

fn parse_contraption(input: Vec<String>) -> Contraption {
    input
        .iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut contraption, (y, line)| {
            line.chars().enumerate().for_each(|(x, field)| {
                let field = match field {
                    '.' => Field::Empty,
                    '\\' => Field::RightDownMirror,
                    '/' => Field::RightUpMirror,
                    '|' => Field::VerticalSplitter,
                    '-' => Field::HorizontalSplitter,
                    _ => panic!("Not a valid Field"),
                };
                contraption.insert(Vec2D::new(x as i32, y as i32), field);
            });
            contraption
        })
}

fn light_contraption(contraption: Contraption, starting_beam: Beam) -> u32 {
    let mut beams = vec![starting_beam];
    let mut visited = HashSet::new();
    let mut num_last_visited = 0;

    // Check first field
    if let Some(field) = contraption.get(&beams[0].head) {
        match field {
            Field::Empty => {
                visited.insert((beams[0].head, beams[0].direction));
            }
            Field::RightDownMirror => {
                beams[0].rotate(field);
                visited.insert((beams[0].head, beams[0].direction));
            }
            Field::RightUpMirror => {
                beams[0].rotate(field);
                visited.insert((beams[0].head, beams[0].direction));
            }
            Field::VerticalSplitter => {
                if [Direction::Left, Direction::Right].contains(&beams[0].direction) {
                    beams[0].direction = Direction::Up;
                    beams.push(Beam::new(beams[0].head, Direction::Down));
                }
                visited.insert((beams[0].head, beams[0].direction));
            }
            Field::HorizontalSplitter => {
                if [Direction::Up, Direction::Down].contains(&beams[0].direction) {
                    beams[0].direction = Direction::Left;
                    beams.push(Beam::new(beams[0].head, Direction::Right));
                }
                visited.insert((beams[0].head, beams[0].direction));
            }
        }
    }

    while num_last_visited < visited.len() {
        num_last_visited = visited.len();
        beams = beams
            .clone()
            .into_iter()
            .fold(vec![], |mut beams, mut beam| {
                beam.next();
                if let Some(field) = contraption.get(&beam.head) {
                    match field {
                        Field::Empty => {
                            visited.insert((beam.head, beam.direction));
                            beams.push(beam);
                        }
                        Field::RightDownMirror => {
                            beam.rotate(field);
                            visited.insert((beam.head, beam.direction));
                            beams.push(beam);
                        }
                        Field::RightUpMirror => {
                            beam.rotate(field);
                            visited.insert((beam.head, beam.direction));
                            beams.push(beam);
                        }
                        Field::VerticalSplitter => {
                            if [Direction::Left, Direction::Right].contains(&beam.direction) {
                                beam.direction = Direction::Up;
                                beams.push(Beam::new(beam.head, Direction::Down));
                            }
                            visited.insert((beam.head, beam.direction));
                            beams.push(beam);
                        }
                        Field::HorizontalSplitter => {
                            if [Direction::Up, Direction::Down].contains(&beam.direction) {
                                beam.direction = Direction::Left;
                                beams.push(Beam::new(beam.head, Direction::Right));
                            }
                            visited.insert((beam.head, beam.direction));
                            beams.push(beam);
                        }
                    }
                };
                beams
            });
    }

    visited.iter().map(|(pos, _)| pos).unique().count() as u32
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let contraption = parse_contraption(input);

    Ok(light_contraption(contraption, Beam::start()))
}

fn part2(path: &str) -> Result<u32> {
    let input = aoc23::read_one_per_line::<String>(path)?;
    let x_len = input.clone()[0].len();
    let y_len = input.clone().len();

    let contraption = parse_contraption(input);

    let starting_beams = (0..x_len)
        .map(|x| Beam::new(Vec2D::new(x as i32, 0), Direction::Down))
        .chain((0..x_len).map(|x| Beam::new(Vec2D::new(x as i32, x_len as i32 - 1), Direction::Up)))
        .chain((0..y_len).map(|y| Beam::new(Vec2D::new(0, y as i32), Direction::Right)))
        .chain(
            (0..y_len).map(|y| Beam::new(Vec2D::new(y_len as i32 - 1, y as i32), Direction::Left)),
        )
        .collect_vec();

    let lit_fields = starting_beams
        .into_iter()
        .map(|starting_beam| light_contraption(contraption.clone(), starting_beam));

    Ok(lit_fields.max().expect("Should have max"))
}

fn main() {
    println!("Part1: {}", part1("data/16.input").unwrap());
    println!("Part2: {}", part2("data/16.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/16.sample").unwrap(), 46);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/16.sample").unwrap(), 51);
    }
}
