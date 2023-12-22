use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    IResult,
};
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { state: Pulse },
    Conjuction { memory: HashMap<&'a str, Pulse> },
}

type From = String;
type To = String;

#[derive(Debug, Clone)]
struct Module<'a> {
    id: &'a str,
    output: Vec<&'a str>,
    module_type: ModuleType<'a>,
}

impl<'a> Module<'a> {
    fn process(&mut self, sender_id: From, pulse: &Pulse) -> Vec<(From, To, Pulse)> {
        match &mut self.module_type {
            ModuleType::Broadcaster => self
                .output
                .iter()
                .map(|&id| (self.id.to_string(), id.to_string(), *pulse))
                .collect::<Vec<(From, To, Pulse)>>(),
            ModuleType::FlipFlop { state } => match (pulse, &state) {
                (Pulse::High, _) => vec![],
                (Pulse::Low, Pulse::High) => {
                    *state = Pulse::Low;
                    self.output
                        .iter()
                        .map(|&id| (self.id.to_string(), id.to_string(), Pulse::Low))
                        .collect::<Vec<(From, To, Pulse)>>()
                }
                (Pulse::Low, Pulse::Low) => {
                    *state = Pulse::High;
                    self.output
                        .iter()
                        .map(|&id| (self.id.to_string(), id.to_string(), Pulse::High))
                        .collect::<Vec<(From, To, Pulse)>>()
                }
            },
            ModuleType::Conjuction { memory } => {
                *memory.get_mut(sender_id.as_str()).expect("Should exist") = *pulse;
                let new_pulse = if memory.values().all(|value| matches!(value, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                self.output
                    .iter()
                    .map(|&id| (self.id.to_string(), id.to_string(), new_pulse))
                    .collect::<Vec<(From, To, Pulse)>>()
            }
        }
    }
}

fn broadcaster(input: &str) -> IResult<&str, Module> {
    let (input, _) = tag("broadcaster -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Module {
            id: "broadcaster",
            output: outputs,
            module_type: ModuleType::Broadcaster,
        },
    ))
}

fn flip_flop(input: &str) -> IResult<&str, Module> {
    let (input, _) = tag("%")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Module {
            id: name,
            output: outputs,
            module_type: ModuleType::FlipFlop { state: Pulse::Low },
        },
    ))
}

fn conjuction(input: &str) -> IResult<&str, Module> {
    let (input, _) = tag("&")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Module {
            id: name,
            output: outputs,
            module_type: ModuleType::Conjuction {
                memory: HashMap::new(),
            },
        },
    ))
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Module>> {
    let (input, modules) =
        separated_list1(line_ending, alt((broadcaster, flip_flop, conjuction)))(input)?;

    Ok((
        input,
        modules
            .into_iter()
            .map(|module| (module.id, module))
            .collect(),
    ))
}

fn add_inputs<'a>(mut modules: HashMap<&'a str, Module<'a>>) -> HashMap<&'a str, Module<'a>> {
    let conjuctions = modules
        .iter()
        .filter_map(|(id, module)| match &module.module_type {
            ModuleType::Broadcaster => None,
            ModuleType::FlipFlop { .. } => None,
            ModuleType::Conjuction { .. } => Some(*id),
        })
        .collect::<Vec<&str>>();

    let inputs = modules.iter().fold(
        HashMap::<&str, Vec<&str>>::new(),
        |mut acc, (id, module)| {
            for c in conjuctions.iter() {
                if module.output.contains(c) {
                    acc.entry(c)
                        .and_modify(|item| item.push(id))
                        .or_insert(vec![id]);
                }
            }
            acc
        },
    );

    inputs.into_iter().for_each(|(con, input_modules)| {
        modules.entry(con).and_modify(|module| {
            let ModuleType::Conjuction { memory, .. } = &mut module.module_type else {
                unreachable!("Has to be a conjuction");
            };
            *memory = input_modules
                .into_iter()
                .map(|id| (id, Pulse::Low))
                .collect();
        });
    });

    modules
}

fn part1(path: &str) -> Result<usize> {
    let input = aoc23::load_input(path)?;
    let (_input, modules) = parse(&input).expect("Should parse");
    let mut modules = add_inputs(modules);

    let mut button_presses = 0;
    let mut cycle_pulses = vec![];
    loop {
        button_presses += 1;
        let mut pulses = (0, 1);
        let mut inbox = VecDeque::<(From, To, Pulse)>::from([(
            "button".to_string(),
            "broadcaster".to_string(),
            Pulse::Low,
        )]);
        while let Some((from, to, pulse)) = inbox.pop_front() {
            let outputs = modules
                .get_mut(to.as_str())
                .map(|m| m.process(from.clone(), &pulse))
                .unwrap_or_default();
            for (_, _, pulse) in outputs.iter() {
                match pulse {
                    Pulse::High => pulses.0 += 1,
                    Pulse::Low => pulses.1 += 1,
                }
            }

            inbox.extend(outputs);
        }

        cycle_pulses.push(pulses);

        if modules
            .iter()
            .filter(|(_, module)| {
                if let ModuleType::FlipFlop { state } = &module.module_type {
                    matches!(state, Pulse::High)
                } else {
                    false
                }
            })
            .collect_vec()
            .is_empty()
            || button_presses == 1000
        {
            break;
        }
    }

    let (highs, lows): (Vec<usize>, Vec<usize>) = cycle_pulses.into_iter().unzip();
    let high_sums = highs.clone().into_iter().sum::<usize>();
    let low_sums = lows.clone().into_iter().sum::<usize>();

    let cycles = 1000 / button_presses;
    let left_presses = 1000 % button_presses;

    let left_highs = highs.into_iter().take(left_presses).sum::<usize>();
    let left_lows = lows.into_iter().take(left_presses).sum::<usize>();

    let highs_full = high_sums * cycles + left_highs;
    let lows_full = low_sums * cycles + left_lows;

    Ok(highs_full * lows_full)
}

fn part2(path: &str) -> Result<usize> {
    let input = aoc23::load_input(path)?;
    let (_input, modules) = parse(&input).expect("Should parse");
    let mut modules = add_inputs(modules);

    // This is a conjuction

    let check_modules = modules.clone();

    let rx_input = check_modules
        .values()
        .find(|module| module.output.contains(&"rx"))
        .expect("Should exist");

    let mut input_inputs = check_modules
        .values()
        .filter_map(|module| {
            if module.output.contains(&rx_input.id) {
                Some(module.id)
            } else {
                None
            }
        })
        .collect_vec();

    let mut lcms = vec![];
    for i in 0.. {
        if lcms.len() == 4 {
            break;
        }

        let mut inbox = VecDeque::<(From, To, Pulse)>::from([(
            "button".to_string(),
            "broadcaster".to_string(),
            Pulse::Low,
        )]);
        while let Some((from, to, pulse)) = inbox.pop_front() {
            if input_inputs.contains(&to.as_str()) && matches!(pulse, Pulse::Low) {
                let index = input_inputs
                    .iter()
                    .position(|x| x == &to)
                    .expect("Should exist");

                input_inputs.remove(index);

                lcms.push(i + 1)
            }
            let outputs = modules
                .get_mut(to.as_str())
                .map(|m| m.process(from.clone(), &pulse))
                .unwrap_or_default();

            inbox.extend(outputs);
        }
    }

    let mut result = 1;
    lcms.into_iter().for_each(|presses| {
        result = lcm(result, presses);
    });

    Ok(result)
}

fn main() {
    println!("Part1: {}", part1("data/20.input").unwrap());
    println!("Part2: {}", part2("data/20.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("data/20_1.sample", 32000000 ; "First sample")]
    #[test_case("data/20_2.sample", 11687500 ; "Second sample")]
    fn part1_test(path: &str, result: usize) {
        assert_eq!(part1(path).unwrap(), result);
    }

    // No test for part 2 :(
}
