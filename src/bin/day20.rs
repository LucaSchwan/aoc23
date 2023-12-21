use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

fn name_to_id(name: String) -> usize {
    let mut id = 0;
    for (exp, byte) in name.bytes().enumerate() {
        id += (byte as usize) * 256_usize.pow(exp as u32);
    }
    id
}

const HIGH: bool = true;
const LOW: bool = false;

#[derive(Debug)]
struct Pulse {
    from: usize,
    to: usize,
    pulse_type: bool,
}

#[derive(Debug)]
struct PulseQueue {
    queue: VecDeque<Pulse>,
    low_count: usize,
    high_count: usize,
    button_count: usize,
}

impl PulseQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            low_count: 0,
            high_count: 0,
            button_count: 0,
        }
    }

    fn send(&mut self, from: usize, pulse_type: bool, targets: &[usize]) {
        targets.iter().for_each(move |target| {
            match pulse_type {
                HIGH => self.high_count += 1,
                LOW => self.low_count += 1,
            }

            self.queue.push_back(Pulse {
                from,
                pulse_type,
                to: *target,
            })
        })
    }

    fn push_button(&mut self, modules: &mut ModulesGraph) {
        self.low_count += 1;
        self.button_count += 1;
        self.queue.push_back(Pulse {
            from: 0,
            to: 0,
            pulse_type: LOW,
        });

        while let Some(pulse) = self.queue.pop_front() {
            if let Some(module) = modules.get_mut(&pulse.to) {
                module.receive(pulse, self);
            }
        }
    }
}

type ModulesGraph = HashMap<usize, Box<dyn Module>>;

trait Module {
    fn receive(&mut self, pulse: Pulse, pulse_queue: &mut PulseQueue);
    fn flip_flop_state(&self) -> Option<bool>;
    fn get_nexts(&self) -> Vec<usize>;
    fn get_id(&self) -> usize;
}

impl Debug for dyn Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Module: {:?}, Nexts: {:?}",
            self.get_id(),
            self.get_nexts()
        )
    }
}

#[derive(Debug)]
struct FlipFlop {
    id: usize,
    state: bool,
    nexts: Vec<usize>,
}

impl FlipFlop {
    fn new(id: usize, nexts: Vec<usize>) -> Self {
        Self {
            id,
            state: LOW,
            nexts,
        }
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, pulse: Pulse, pulse_queue: &mut PulseQueue) {
        if matches!(pulse.pulse_type, LOW) {
            self.state = !self.state;

            pulse_queue.send(self.id, self.state, &self.nexts)
        }
    }

    fn flip_flop_state(&self) -> Option<bool> {
        Some(self.state)
    }

    fn get_nexts(&self) -> Vec<usize> {
        self.nexts.clone()
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
struct Conjuction {
    id: usize,
    inputs: HashMap<usize, bool>,
    nexts: Vec<usize>,
}

impl Conjuction {
    fn new(id: usize, input_names: Vec<usize>, nexts: Vec<usize>) -> Self {
        let mut inputs = HashMap::new();

        input_names.into_iter().for_each(|name| {
            inputs.insert(name, LOW);
        });

        Self { id, inputs, nexts }
    }
}

impl Module for Conjuction {
    fn receive(&mut self, pulse: Pulse, pulse_queue: &mut PulseQueue) {
        self.inputs
            .entry(pulse.from)
            .and_modify(|i| *i = pulse.pulse_type);

        if self.inputs.iter().all(|(_, pulse)| *pulse) {
            pulse_queue.send(self.id, LOW, &self.nexts)
        } else {
            pulse_queue.send(self.id, HIGH, &self.nexts)
        }
    }

    fn flip_flop_state(&self) -> Option<bool> {
        None
    }

    fn get_nexts(&self) -> Vec<usize> {
        self.nexts.clone()
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
struct Broadcast {
    id: usize,
    nexts: Vec<usize>,
}

impl Broadcast {
    fn new(id: usize, nexts: Vec<usize>) -> Self {
        Self { id, nexts }
    }
}

impl Module for Broadcast {
    fn receive(&mut self, pulse: Pulse, pulse_queue: &mut PulseQueue) {
        pulse_queue.send(self.id, pulse.pulse_type, &self.nexts);
    }

    fn flip_flop_state(&self) -> Option<bool> {
        None
    }

    fn get_nexts(&self) -> Vec<usize> {
        self.nexts.clone()
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

fn parse_graph(connections: Vec<String>) -> ModulesGraph {
    connections
        .into_iter()
        .fold(HashMap::new(), |mut graph, conn| {
            let (name, next) = conn.split_once(" -> ").expect("Should be well formed");

            let name = name.to_string();
            let next = next
                .split(", ")
                .map(|s| name_to_id(s.to_string()))
                .collect();

            match name.chars().next().expect("Should have first character") {
                'b' => graph.insert(0, Box::new(Broadcast::new(0, next))),
                '%' => {
                    let id = name_to_id(name.chars().skip(1).collect());
                    graph.insert(id, Box::new(FlipFlop::new(id, next)))
                }
                '&' => {
                    let id = name_to_id(name.chars().skip(1).collect());
                    let inputs = graph
                        .iter()
                        .filter(|(_, node)| node.get_nexts().contains(&id))
                        .map(|(id, _)| *id)
                        .collect::<Vec<usize>>();
                    graph.insert(id, Box::new(Conjuction::new(id, inputs, next)))
                }
                _ => unreachable!(),
            };
            graph
        })
}

fn part1(path: &str) -> Result<usize> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let mut graph = parse_graph(input);

    let mut pulse_queue = PulseQueue::new();
    let mut pulse_counts = vec![(0, 0)];

    loop {
        pulse_queue.push_button(&mut graph);
        pulse_counts.push((pulse_queue.high_count, pulse_queue.low_count));

        if graph
            .iter()
            .filter_map(|(_, module)| module.flip_flop_state())
            .all(|state| !state)
            || pulse_queue.button_count == 1000
        {
            break;
        }
    }

    let (cycle_highs, cycle_lows): (Vec<_>, Vec<_>) = pulse_counts.clone().into_iter().unzip();

    dbg!(&pulse_queue);

    let high = (1000 / pulse_queue.button_count) * pulse_queue.high_count
        + cycle_highs[1000 % pulse_queue.button_count];
    let low = (1000 / pulse_queue.button_count) * pulse_queue.low_count
        + cycle_lows[1000 % pulse_queue.button_count];

    Ok(high * low)
}

fn part2(_path: &str) -> Result<u32> {
    todo!()
}

fn main() {
    println!("Part1: {}", part1("data/20.input").unwrap());
    todo!();
    println!("Part1: {}", part2("data/x.input").unwrap());
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

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2("data/x.sample").unwrap(), 0);
    // }
}
