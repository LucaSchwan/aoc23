use anyhow::Result;
use aoc23::Vec2D;
use std::collections::{BTreeMap, HashSet};

#[derive(Clone)]
enum Position {
    Starting,
    GardenPlot,
    Rocks,
}

type Garden = BTreeMap<Vec2D, Position>;

fn parse_garden(input: &str) -> (Garden, Vec2D) {
    let mut starting = Vec2D::ZERO;
    (
        input
            .lines()
            .enumerate()
            .fold(BTreeMap::new(), |mut garden, (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    let position = match c {
                        'S' => Position::Starting,
                        '.' => Position::GardenPlot,
                        '#' => Position::Rocks,
                        _ => unreachable!(),
                    };

                    let coords = Vec2D::new(x as i32, y as i32);

                    if matches!(position, Position::Starting) {
                        starting = coords;
                    }

                    garden.insert(coords, position);
                });
                garden
            }),
        starting,
    )
}

fn part1(path: &str, steps: usize) -> Result<usize> {
    let input = aoc23::load_input(path)?;
    let (garden, starting) = parse_garden(&input);

    let mut poss = HashSet::new();
    poss.insert(starting);

    for _ in 0..steps {
        poss = poss.into_iter().fold(HashSet::new(), |mut poss, curr| {
            [Vec2D::UP, Vec2D::RIGHT, Vec2D::DOWN, Vec2D::LEFT]
                .into_iter()
                .for_each(|offset| {
                    if let Some(Position::Rocks) = garden.get(&(curr + offset)) {
                        return;
                    }

                    poss.insert(curr + offset);
                });
            poss
        });
    }

    Ok(poss.len())
}

fn step_once_infinite_grid(
    garden: Garden,
    garden_x: i32,
    garden_y: i32,
    poss: HashSet<Vec2D>,
) -> HashSet<Vec2D> {
    poss.into_iter().fold(HashSet::new(), |mut poss, curr| {
        [Vec2D::UP, Vec2D::RIGHT, Vec2D::DOWN, Vec2D::LEFT]
            .into_iter()
            .for_each(|offset| {
                let new_pos = curr + offset;
                let mut check_pos = new_pos;

                if check_pos.x > garden_x {
                    check_pos.x %= garden_x;
                }

                if check_pos.y > garden_y {
                    check_pos.y %= garden_y;
                }

                if check_pos.x < 0 {
                    check_pos.x = check_pos.x.abs() % garden_x;
                }

                if check_pos.y < 0 {
                    check_pos.y = check_pos.y.abs() % garden_y;
                }

                if let Some(Position::Rocks) = garden.get(&check_pos) {
                    return;
                }

                poss.insert(new_pos);
            });
        poss
    })
}

fn calc_xs_ys(path: &str) -> Result<Vec<(usize, usize)>> {
    let input = aoc23::load_input(path)?;
    let (garden, starting) = parse_garden(&input);

    let garden_x = garden
        .clone()
        .into_keys()
        .map(|p| p.x)
        .max()
        .expect("Should have max");
    let garden_y = garden
        .clone()
        .into_keys()
        .map(|p| p.y)
        .max()
        .expect("Should have max");

    let mut interpolation = vec![];

    let mut poss = HashSet::new();
    poss.insert(starting);

    for _ in 0..65 {
        poss = step_once_infinite_grid(garden.clone(), garden_x, garden_y, poss);
    }

    interpolation.push((65, poss.len()));

    for _ in 0..131 {
        poss = step_once_infinite_grid(garden.clone(), garden_x, garden_y, poss);
    }

    interpolation.push((65 + 131, poss.len()));

    for _ in 0..131 {
        poss = step_once_infinite_grid(garden.clone(), garden_x, garden_y, poss);
    }

    interpolation.push((65 + 131 * 2, poss.len()));

    for _ in 0..131 {
        poss = step_once_infinite_grid(garden.clone(), garden_x, garden_y, poss);
    }

    interpolation.push((65 + 131 * 3, poss.len()));

    Ok(interpolation)
}

fn part2(path: &str, steps: usize) -> Result<f64> {
    // let (_, ys): (Vec<_>, Vec<_>) = calc_xs_ys(path)?.into_iter().unzip();
    //
    // println!(
    //     "{{{{{}, {}}}, {{{}, {}}}, {{{}, {}}}, {{{}, {}}}}}",
    //     0, ys[0], 1, ys[1], 2, ys[2], 3, ys[3]
    // );
    //
    // let first_diff1 = ys[1] - ys[0];
    // let first_diff2 = ys[2] - ys[1];
    // let second_diff = first_diff2 - first_diff1;
    //
    // let a = second_diff / 2;
    // let b = first_diff1 - 3 * a;
    // let c = ys[0] - b - a;
    //
    // let poly = |n: usize| a * n.pow(2) + b * n + c;

    let x = steps as f64 - 65.0 / 131.0;
    //3712.3 + 15097.3 x + 14706.5 x^2
    let result = 3712.3 + 15097.3 * x + 14706.5 * x.powf(2.0);

    Ok(result)
}

fn main() {
    println!("Part1: {}", part1("data/21.input", 64).unwrap());
    println!("Part2: {}", part2("data/21.input", 26501365).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/21.sample", 6).unwrap(), 16);
    }
}
