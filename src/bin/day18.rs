use anyhow::Result;
use aoc23::Vec2D;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{self, newline, one_of, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser,
};
use std::collections::BTreeMap;

#[derive(Clone)]
enum HolePart {
    Edge,
    Hole,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type DirectionSwapped = (Direction, u32);
type DiggingInstruction = (Direction, u32, DirectionSwapped);
type DiggingPlan = Vec<DiggingInstruction>;
type Hole = BTreeMap<Vec2D, HolePart>;

fn hexdigit(input: &str) -> IResult<&str, u32> {
    take_while_m_n(1, 5, |c: char| c.is_ascii_hexdigit())
        .map(|c| u32::from_str_radix(c, 16).expect("Should be hex"))
        .parse(input)
}

fn direction_swapped(input: &str) -> IResult<&str, DirectionSwapped> {
    let (input, (distance, direction)) = delimited(
        tag("(#"),
        tuple((
            hexdigit,
            complete::u8.map(|num| match num {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => unreachable!(),
            }),
        )),
        tag(")"),
    )(input)?;

    Ok((input, (direction, distance)))
}

fn instruction(input: &str) -> IResult<&str, DiggingInstruction> {
    tuple((
        one_of("URDL").map(|c| match c {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => unreachable!(),
        }),
        space1,
        complete::u32,
        space1,
        direction_swapped,
    ))
    .map(|(d, _, n, _, c)| (d, n, c))
    .parse(input)
}

fn parse_digging_plan(input: &str) -> IResult<&str, DiggingPlan> {
    let (input, plan) = separated_list1(newline, instruction)(input)?;

    Ok((input, plan))
}

fn create_hole_boundaries(plan: DiggingPlan) -> Hole {
    let mut hole = BTreeMap::new();
    hole.insert(Vec2D::ZERO, HolePart::Edge);

    let (hole, _) = plan.into_iter().fold(
        (hole, Vec2D::ZERO),
        |(mut hole, mut current), instruction| {
            let direction = instruction.0;
            let distance = instruction.1;
            (1..=distance).for_each(|_| {
                match direction {
                    Direction::Up => current.y -= 1,
                    Direction::Right => current.x += 1,
                    Direction::Down => current.y += 1,
                    Direction::Left => current.x -= 1,
                }

                hole.insert(current, HolePart::Edge);
            });
            (hole, current)
        },
    );

    hole
}

fn flood_fill(mut hole: Hole) -> Hole {
    let mut currents = vec![Vec2D::new(1, 1)];

    while !currents.is_empty() {
        while let Some(current) = currents.pop() {
            hole.insert(current, HolePart::Hole);

            let mut neighbours = vec![
                current - Vec2D::UP,
                current - Vec2D::RIGHT,
                current - Vec2D::DOWN,
                current - Vec2D::LEFT,
            ];

            neighbours.retain(|neighbour| hole.get(neighbour).is_none());

            currents.append(&mut neighbours);
        }
    }

    hole
}

fn calc_vertices(plan: DiggingPlan) -> (Vec<Vec2D>, i64) {
    let vertices = vec![Vec2D::ZERO];
    let mut boundary: i64 = 0;

    let (vertices, _) = plan.into_iter().fold(
        (vertices, Vec2D::ZERO),
        |(mut vertices, mut current), instruction| {
            let direction = instruction.2 .0;
            let steps = instruction.2 .1;
            match direction {
                Direction::Up => current.y -= steps as i32,
                Direction::Right => current.x += steps as i32,
                Direction::Down => current.y += steps as i32,
                Direction::Left => current.x -= steps as i32,
            }

            boundary += steps as i64;
            vertices.push(current);
            (vertices, current)
        },
    );

    (vertices, boundary)
}

fn shoelace_theorem(vertices: Vec<Vec2D>, boundary: i64) -> i64 {
    let a = vertices
        .clone()
        .into_iter()
        .zip(vertices.into_iter().cycle().skip(1))
        .map(|(a, b)| a.x as i64 * b.y as i64 - a.y as i64 * b.x as i64)
        .sum::<i64>();

    a.abs() / 2 + boundary / 2 + 1
}

fn _print_hole(hole: &Hole) {
    let (x_len, y_len) = hole
        .iter()
        .map(|(Vec2D { x, y }, _)| (x, y))
        .max()
        .expect("Should have max");

    (0..=*y_len).for_each(|y| {
        (0..=*x_len).for_each(|x| {
            if hole.get(&Vec2D::new(x, y)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    })
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;

    let (_, plan) = parse_digging_plan(&input).expect("Should parse");
    let hole = create_hole_boundaries(plan);
    let hole = flood_fill(hole);

    Ok(hole.len() as u32)
}

fn part2(path: &str) -> Result<i64> {
    let input = aoc23::load_input(path)?;

    let (_, plan) = parse_digging_plan(&input).expect("Should parse");
    let (vertices, boundary) = calc_vertices(plan);

    Ok(shoelace_theorem(vertices, boundary))
}

fn main() {
    println!("Part1: {}", part1("data/18.input").unwrap());
    println!("Part2: {}", part2("data/18.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/18.sample").unwrap(), 62);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/18.sample").unwrap(), 952408144115);
    }
}
