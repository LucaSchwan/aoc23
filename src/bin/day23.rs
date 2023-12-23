use anyhow::Result;
use aoc23::Vec2D;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: Vec2D,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Ground {
    Path,
    Forest,
    UpSlope,
    RightSlope,
    DownSlope,
    LeftSlope,
}

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
struct Edge {
    node: Vec2D,
    cost: isize,
}

type HikingPaths = HashMap<Vec2D, Node>;

fn parse_paths(input: Vec<String>) -> (HikingPaths, Vec2D) {
    let mut parsed_ground: HashMap<_, _> = HashMap::new();

    input.into_iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            use Ground::*;

            let ground = match c {
                '.' => Path,
                '#' => Forest,
                '^' => UpSlope,
                '>' => RightSlope,
                'v' => DownSlope,
                '<' => LeftSlope,
                _ => unreachable!("Wrong ground type"),
            };
            let position = Vec2D::new(x as i32, y as i32);

            parsed_ground.insert(position, ground);
        })
    });

    let xlen = parsed_ground
        .keys()
        .map(|Vec2D { x, .. }| x)
        .max()
        .expect("Should have max");
    let ylen = parsed_ground
        .keys()
        .map(|Vec2D { y, .. }| y)
        .max()
        .expect("Should have max");

    let start = Vec2D::new(1, 0);
    let end = Vec2D::new(xlen - 1, *ylen);

    let mut paths: HashMap<Vec2D, Node> = HashMap::new();
    paths.insert(start, Node { edges: vec![] });

    calc_next_node(&parsed_ground, start, start, end, &mut paths);

    paths
        .entry(start)
        .and_modify(|node| node.edges[0].cost -= 1);

    (paths, start)
}

fn calc_next_node(
    ground: &HashMap<Vec2D, Ground>,
    starting_node: Vec2D,
    prev_node: Vec2D,
    end: Vec2D,
    paths: &mut HikingPaths,
) {
    let mut current = (starting_node, prev_node);
    let mut found_slope = None;
    let mut cost = 2; // Because the slope and start get's skipped

    while found_slope.is_none() {
        for offset in [Vec2D::UP, Vec2D::RIGHT, Vec2D::DOWN, Vec2D::LEFT] {
            let check_pos = current.0 + offset;

            if check_pos == current.1 {
                continue;
            }

            if check_pos == end {
                paths.insert(check_pos, Node { edges: vec![] });

                paths.entry(prev_node).and_modify(|node| {
                    node.edges.push(Edge {
                        node: check_pos,
                        cost,
                    })
                });
                return;
            }

            if let Some(ground_type) = ground.get(&check_pos) {
                match ground_type {
                    Ground::Path => {
                        cost += 1;
                        current = (check_pos, current.0)
                    }
                    Ground::Forest => {}
                    slope => {
                        cost += 1;
                        found_slope = Some((check_pos, slope))
                    }
                }
            }
        }
    }

    let (slope_pos, slope_type) = found_slope.expect("Should be Some at this point");

    let mut next_node_pos = slope_pos;
    match slope_type {
        Ground::UpSlope => next_node_pos = next_node_pos + Vec2D::UP,
        Ground::RightSlope => next_node_pos = next_node_pos + Vec2D::RIGHT,
        Ground::DownSlope => next_node_pos = next_node_pos + Vec2D::DOWN,
        Ground::LeftSlope => next_node_pos = next_node_pos + Vec2D::LEFT,
        _ => unreachable!("Should only be a slope"),
    }

    paths.insert(next_node_pos, Node { edges: vec![] });

    paths.entry(prev_node).and_modify(|node| {
        node.edges.push(Edge {
            node: next_node_pos,
            cost,
        })
    });

    let next_starting_nodes = [Vec2D::UP, Vec2D::RIGHT, Vec2D::DOWN, Vec2D::LEFT]
        .into_iter()
        .filter_map(|offset| {
            let check_pos = next_node_pos + offset;
            if check_pos == slope_pos {
                return None;
            }

            match ground.get(&check_pos).expect("Should exist") {
                Ground::UpSlope => {
                    if offset != Vec2D::DOWN {
                        Some(check_pos)
                    } else {
                        None
                    }
                }
                Ground::RightSlope => {
                    if offset != Vec2D::LEFT {
                        Some(check_pos)
                    } else {
                        None
                    }
                }
                Ground::DownSlope => {
                    if offset != Vec2D::UP {
                        Some(check_pos)
                    } else {
                        None
                    }
                }
                Ground::LeftSlope => {
                    if offset != Vec2D::RIGHT {
                        Some(check_pos)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        });

    next_starting_nodes.for_each(|starting_node| {
        calc_next_node(ground, starting_node, next_node_pos, end, paths);
    })
}

fn find_longest_path(paths: HikingPaths, start: Vec2D) -> isize {
    let mut distances: HashMap<_, _> = paths.keys().map(|pos| (pos, isize::MIN)).collect();
    distances.entry(&start).and_modify(|dist| *dist = 0);

    let mut visited: HashMap<_, _> = paths.keys().map(|pos| (*pos, false)).collect();
    let mut stack = vec![];

    topological_sort(start, paths.clone(), &mut visited, &mut stack);

    stack.reverse();

    stack.into_iter().for_each(|node_pos| {
        let node = paths.get(&node_pos).expect("Should exist");
        let node_dist = *distances.get(&node_pos).expect("Should exist");
        node.edges.iter().for_each(|edge| {
            let edge_dist = distances.get(&edge.node).expect("Should exist");
            let new_edge_dist = std::cmp::max(*edge_dist, node_dist + edge.cost);
            *distances.get_mut(&edge.node).expect("Should exist") = new_edge_dist;
        })
    });

    *distances.values().max().expect("Should have max")
}

fn topological_sort(
    position: Vec2D,
    paths: HikingPaths,
    visited: &mut HashMap<Vec2D, bool>,
    stack: &mut Vec<Vec2D>,
) {
    visited.entry(position).and_modify(|seen| *seen = true);

    for edge in &paths.get(&position).expect("Should exist").edges {
        if !visited.get(&edge.node).expect("Should exist") {
            topological_sort(edge.node, paths.clone(), visited, stack)
        }
    }

    stack.push(position);
}

fn print_graph_for_graphviz(paths: &HikingPaths, directed: bool) {
    let edge_arrow = if directed { "->" } else { "--" };
    let mut nodes = paths.clone().into_iter().collect_vec();
    nodes.sort_by_key(|(pos, _)| *pos);
    if directed {
        println!("digraph {{");
    } else {
        println!("graph {{")
    }
    for node in nodes {
        for edge in node.1.edges {
            println!(
                "  \"{}\" {} \"{}\" [label={}]",
                node.0, edge_arrow, edge.node, edge.cost
            );
        }
    }
    println!("}}");
}

fn part1(path: &str) -> Result<isize> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let (paths, start) = parse_paths(input);

    print_graph_for_graphviz(&paths, false);

    Ok(find_longest_path(paths, start))
}

fn part2(_path: &str) -> Result<u32> {
    todo!()
}

fn main() {
    println!("Part1: {}", part1("data/23.input").unwrap());
    todo!();
    println!("Part1: {}", part2("data/x.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/23.sample").unwrap(), 94);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/23.sample").unwrap(), 0);
    }
}
