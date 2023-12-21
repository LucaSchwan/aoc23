use anyhow::Result;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    fall_through: Next,
}

#[derive(Debug, Clone)]
struct Rule {
    expr: Expr,
    next: Next,
}

#[derive(Debug, Clone)]
enum Expr {
    GT(char, u32),
    ST(char, u32),
}

impl Expr {
    fn eval(&self, part: Part) -> bool {
        match self {
            Expr::GT(c, val) => {
                let cmp = match c {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => unreachable!(),
                };
                cmp > *val
            }
            Expr::ST(c, val) => {
                let cmp = match c {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => unreachable!(),
                };
                cmp < *val
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Next {
    Rejected,
    Accepted,
    Workflow(String),
}

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone)]
struct RangePart {
    parts: BTreeMap<char, RangeInclusive<u32>>,
}

impl RangePart {
    fn new(
        x: RangeInclusive<u32>,
        m: RangeInclusive<u32>,
        a: RangeInclusive<u32>,
        s: RangeInclusive<u32>,
    ) -> Self {
        let mut parts = BTreeMap::new();
        parts.insert('x', x);
        parts.insert('m', m);
        parts.insert('a', a);
        parts.insert('s', s);

        Self { parts }
    }
}

fn parse_input(input: String) -> (BTreeMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").expect("Should be well formed");

    let workflows = workflows
        .lines()
        .fold(BTreeMap::new(), |mut workflows, line| {
            let (name, rules) = line.split_once('{').expect("Should be well formed");
            let mut rules: Vec<String> = rules.split(',').map(|rule| rule.to_string()).collect();

            let fall_through = rules.pop().expect("Should have last").replace('}', "");
            let fall_through = match fall_through.as_str() {
                "A" => Next::Accepted,
                "R" => Next::Rejected,
                workflow => Next::Workflow(workflow.to_string()),
            };

            let rules = rules
                .into_iter()
                .map(|rule| {
                    let (expr, next) = rule.split_once(':').expect("Should be well formed");
                    let next = match next {
                        "A" => Next::Accepted,
                        "R" => Next::Rejected,
                        workflow => Next::Workflow(workflow.to_string()),
                    };

                    if expr.find('>').is_some() {
                        let (param, val) = expr.split_once('>').expect("Should be >");
                        let expr = Expr::GT(
                            param.chars().next().expect("Should exist"),
                            val.parse::<u32>().expect("Should be a number"),
                        );
                        return Rule { expr, next };
                    }

                    let (param, val) = expr.split_once('<').expect("Should be <");
                    let expr = Expr::ST(
                        param.chars().next().expect("Should exist"),
                        val.parse::<u32>().expect("Should be a number"),
                    );
                    Rule { expr, next }
                })
                .collect();

            workflows.insert(
                name.to_string(),
                Workflow {
                    rules,
                    fall_through,
                },
            );
            workflows
        });

    let parts = parts
        .lines()
        .map(|part_txt| {
            let mut part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };

            let mut part_txt = part_txt.to_string();

            part_txt.remove(0);
            part_txt.remove(part_txt.len() - 1);

            part_txt.split(',').for_each(|part_part| {
                let (param, val) = part_part.split_once('=').expect("Should be well formed");
                let val = val.parse::<u32>().expect("Should be a number");

                match param {
                    "x" => part.x = val,
                    "m" => part.m = val,
                    "a" => part.a = val,
                    "s" => part.s = val,
                    _ => unreachable!(),
                }
            });

            part
        })
        .collect();

    (workflows, parts)
}

fn execute_workflow(part: Part, workflow: Workflow) -> Next {
    let mut next = None;
    let mut rule_index = 0;
    let rules = workflow.rules;

    while next.is_none() {
        if rule_index == rules.len() {
            next = Some(workflow.fall_through.clone());
            break;
        }
        let rule = &rules[rule_index];
        if rule.expr.eval(part.clone()) {
            next = Some(rule.next.clone());
        }
        rule_index += 1;
    }

    if let Some(next) = next {
        return next;
    }

    Next::Rejected
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;
    let (worklows, parts) = parse_input(input);

    Ok(parts
        .into_iter()
        .filter(|part| {
            let mut next = Next::Workflow("in".to_string());

            while let Next::Workflow(current) = next {
                let workflow = worklows.get(&current).expect("Should exist");

                next = execute_workflow(part.clone(), workflow.clone());
            }

            matches!(next, Next::Accepted)
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u32>())
}

fn part2(path: &str) -> Result<u64> {
    let input = aoc23::load_input(path)?;
    let (worklows, _) = parse_input(input);

    let mut ranges = vec![(
        "in".to_string(),
        0,
        RangePart::new(1u32..=4000, 1u32..=4000, 1u32..=4000, 1u32..=4000),
    )];

    let mut accepted = vec![];

    while let Some(range) = ranges.pop() {
        let workflow = worklows.get(&range.0).expect("Should exist");

        if range.1 == workflow.rules.len() {
            match &workflow.fall_through {
                Next::Rejected => (),
                Next::Accepted => accepted.push(range.2),
                Next::Workflow(workflow_ident) => {
                    ranges.push((workflow_ident.to_string(), 0, range.2))
                }
            }
        } else {
            let rule = &workflow.rules[range.1];

            match rule.expr {
                Expr::GT(param, val) => {
                    let mut matching_range = range.2.clone();
                    matching_range
                        .parts
                        .entry(param)
                        .and_modify(|range| *range = (val + 1)..=*range.end());
                    let mut not_matching_range = range.2.clone();
                    not_matching_range
                        .parts
                        .entry(param)
                        .and_modify(|range| *range = *range.start()..=val);
                    ranges.push((range.0, range.1 + 1, not_matching_range));

                    match &rule.next {
                        Next::Rejected => (),
                        Next::Accepted => accepted.push(matching_range),
                        Next::Workflow(workflow_ident) => {
                            ranges.push((workflow_ident.to_string(), 0, matching_range))
                        }
                    }
                }
                Expr::ST(param, val) => {
                    let mut matching_range = range.2.clone();
                    matching_range
                        .parts
                        .entry(param)
                        .and_modify(|range| *range = *range.start()..=(val - 1));
                    let mut not_matching_range = range.2.clone();
                    not_matching_range
                        .parts
                        .entry(param)
                        .and_modify(|range| *range = val..=*range.end());
                    ranges.push((range.0, range.1 + 1, not_matching_range));

                    match &rule.next {
                        Next::Rejected => (),
                        Next::Accepted => accepted.push(matching_range),
                        Next::Workflow(workflow_ident) => {
                            ranges.push((workflow_ident.to_string(), 0, matching_range))
                        }
                    }
                }
            }
        }
    }

    Ok(accepted
        .into_iter()
        .map(|range| {
            let x = range.parts.get(&'x').expect("Should exist");
            let m = range.parts.get(&'m').expect("Should exist");
            let a = range.parts.get(&'a').expect("Should exist");
            let s = range.parts.get(&'s').expect("Should exist");

            let x = x.end() - x.start() + 1;
            let m = m.end() - m.start() + 1;
            let a = a.end() - a.start() + 1;
            let s = s.end() - s.start() + 1;

            x as u64 * m as u64 * a as u64 * s as u64
        })
        .sum::<u64>())
}

fn main() {
    println!("Part1: {}", part1("data/19.input").unwrap());
    println!("Part2: {}", part2("data/19.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/19.sample").unwrap(), 19114);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/19.sample").unwrap(), 167409079868000);
    }
}
