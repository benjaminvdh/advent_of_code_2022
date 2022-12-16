use std::collections::HashMap;

use crate::{ParseError, SolveError};

#[derive(Clone, Debug, PartialEq)]
pub struct Valve {
    flow_rate: usize,
    tunnels: HashMap<String, usize>,
}

impl Valve {
    fn new<T: AsRef<str>>(flow_rate: usize, tunnels: impl Iterator<Item = T>) -> Self {
        Self {
            flow_rate,
            tunnels: tunnels
                .map(|name| (String::from(name.as_ref()), 1))
                .collect(),
        }
    }
}

pub type Network = HashMap<String, Valve>;

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Network;
    type Output = usize;
    const DAY: u8 = 16;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Result<_, _>>()?)
    }

    fn part_1(network: Self::Input) -> Result<Self::Output, SolveError> {
        let network = collapse(network);

        Ok(get_max_pressure(&network, "AA", 31, &[]))
    }
}

fn collapse(network: Network) -> Network {
    let mut collapsed_network = network.clone();

    for (name, valve) in collapsed_network.iter_mut() {
        valve.tunnels = get_transitive_connections(&network, name.to_owned());
    }

    collapsed_network
}

fn get_transitive_connections(network: &Network, base: String) -> HashMap<String, usize> {
    let mut connections = HashMap::new();

    connections.insert(base, 0);

    for i in 1.. {
        let new: Vec<_> = network
            .iter()
            .filter(|(name, value)| {
                !connections.contains_key(*name)
                    && value.tunnels.keys().any(|t| connections.contains_key(t))
            })
            .map(|(name, _)| (name.to_owned(), i))
            .collect();

        if new.is_empty() {
            break;
        }

        for (a, b) in new {
            connections.insert(a, b);
        }
    }

    connections.retain(|name, value| *value > 0 && network[name].flow_rate > 0);

    connections
}

fn get_max_pressure(
    network: &Network,
    current: &str,
    time_left: usize,
    opened_valves: &[String],
) -> usize {
    let mut new_opened_valves = vec![current.to_owned()];
    new_opened_valves.extend_from_slice(opened_valves);

    (time_left - 1) * network[current].flow_rate
        + network[current]
            .tunnels
            .iter()
            .filter(|(name, _)| opened_valves.iter().find(|t| &t == &name).is_none())
            .map(|(name, distance)| {
                if time_left - 1 > *distance {
                    get_max_pressure(network, name, time_left - 1 - distance, &new_opened_valves)
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0)
}

fn parse_line(line: &str) -> Result<(String, Valve), ParseError> {
    let line = line.trim_start_matches("Valve ");
    let (name, line) = line.split_once(' ').ok_or(ParseError::Invalid)?;
    let line = line.trim_start_matches("has flow rate=");
    let (rate, line) = line.split_once(';').ok_or(ParseError::Invalid)?;
    let line = line.trim_start_matches(" tunnels lead to valves ");
    let line = line.trim_start_matches(" tunnel leads to valve ");
    let tunnels = line.split(", ");

    Ok((name.to_owned(), Valve::new(rate.parse()?, tunnels)))
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::{Network, Valve};

    fn get_input() -> Network {
        let mut network = Network::new();

        network.insert("AA".into(), Valve::new(0, ["DD", "II", "BB"].iter()));
        network.insert("BB".into(), Valve::new(13, ["CC", "AA"].iter()));
        network.insert("CC".into(), Valve::new(2, ["DD", "BB"].iter()));
        network.insert("DD".into(), Valve::new(20, ["CC", "AA", "EE"].iter()));
        network.insert("EE".into(), Valve::new(3, ["FF", "DD"].iter()));
        network.insert("FF".into(), Valve::new(0, ["EE", "GG"].iter()));
        network.insert("GG".into(), Valve::new(0, ["FF", "HH"].iter()));
        network.insert("HH".into(), Valve::new(22, ["GG"].iter()));
        network.insert("II".into(), Valve::new(0, ["AA", "JJ"].iter()));
        network.insert("JJ".into(), Valve::new(21, ["II"].iter()));

        network
    }

    #[test]
    fn parsing() {
        let input = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 1651);
    }
}
