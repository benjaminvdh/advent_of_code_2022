use std::cmp::Ordering;

use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
pub enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Packet::Int(v1) => match other {
                Packet::Int(v2) => v1.partial_cmp(v2),
                Packet::List(_) => Packet::from(vec![*v1]).partial_cmp(other),
            },
            Packet::List(l1) => match other {
                Packet::Int(v2) => self.partial_cmp(&Packet::from(vec![*v2])),
                Packet::List(l2) => {
                    for (p1, p2) in l1.iter().zip(l2.iter()) {
                        if p1 < p2 {
                            return Some(Ordering::Less);
                        } else if p1 > p2 {
                            return Some(Ordering::Greater);
                        }
                    }

                    l1.len().partial_cmp(&l2.len())
                }
            },
        }
    }
}

impl From<u32> for Packet {
    fn from(value: u32) -> Self {
        Packet::Int(value)
    }
}

impl From<Vec<u32>> for Packet {
    fn from(value: Vec<u32>) -> Self {
        Packet::List(value.into_iter().map(|v| v.into()).collect())
    }
}

impl From<Vec<Packet>> for Packet {
    fn from(value: Vec<Packet>) -> Self {
        Packet::List(value)
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<(Packet, Packet)>;
    type Output = usize;
    const DAY: u8 = 13;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        let lines: Vec<_> = input.lines().collect();

        Ok(lines
            .chunks(3)
            .map(|triplet| parse_triplet(triplet))
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        Ok(input
            .iter()
            .enumerate()
            .filter(|(_, (first, second))| first < second)
            .map(|(i, _)| i + 1)
            .sum::<usize>())
    }
}

fn parse_triplet(triplet: &[&str]) -> Result<(Packet, Packet), ParseError> {
    let mut first_i = 1;
    let first = parse_list(triplet[0], &mut first_i)?;

    let mut second_i = 1;
    let second = parse_list(triplet[1], &mut second_i)?;

    Ok((first, second))
}

fn parse_list(line: &str, i: &mut usize) -> Result<Packet, ParseError> {
    let mut packets = vec![];

    while *i < line.len() {
        if &line[*i..*i + 1] == "[" {
            *i += 1;

            packets.push(parse_list(line, i)?);
        } else if &line[*i..*i + 1] == "]" {
            *i += 1;

            return Ok(Packet::List(packets));
        } else if &line[*i..*i + 1] == "," {
            *i += 1;
        } else {
            packets.push(parse_number(line, i)?);
        }
    }

    Err(ParseError::Invalid)
}

fn parse_number(line: &str, i: &mut usize) -> Result<Packet, ParseError> {
    match (line[*i..].find(','), line[*i..].find(']')) {
        (Some(i1), Some(i2)) if i1 < i2 => {
            let packet = Packet::from(line[*i..*i + i1].parse::<u32>()?);
            *i += i1 + 1;
            Ok(packet)
        }
        (Some(i1), None) => {
            let packet = Packet::from(line[*i..*i + i1].parse::<u32>()?);
            *i += i1 + 1;
            Ok(packet)
        }
        (Some(i1), Some(i2)) if i1 > i2 => {
            let packet = Packet::from(line[*i..*i + i2].parse::<u32>()?);
            *i += i2;
            Ok(packet)
        }
        (None, Some(i2)) => {
            let packet = Packet::from(line[*i..*i + i2].parse::<u32>()?);
            *i += i2;
            Ok(packet)
        }
        (Some(_), Some(_)) => unreachable!(),
        (None, None) => Err(ParseError::Invalid),
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::Packet;

    fn get_input() -> Vec<(Packet, Packet)> {
        vec![
            (
                Packet::from(vec![1, 1, 3, 1, 1]),
                Packet::from(vec![1, 1, 5, 1, 1]),
            ),
            (
                Packet::List(vec![Packet::from(vec![1]), Packet::from(vec![2, 3, 4])]),
                Packet::List(vec![Packet::from(vec![1]), Packet::from(4)]),
            ),
            (vec![9].into(), vec![Packet::from(vec![8, 7, 6])].into()),
            (
                vec![Packet::from(vec![4, 4]), 4.into(), 4.into()].into(),
                vec![Packet::from(vec![4, 4]), 4.into(), 4.into(), 4.into()].into(),
            ),
            (vec![7, 7, 7, 7].into(), vec![7, 7, 7].into()),
            (Packet::from(Vec::<u32>::new()), Packet::from(vec![3])),
            (
                vec![Packet::List(vec![Packet::List(vec![])])].into(),
                vec![Packet::List(vec![])].into(),
            ),
            (
                vec![
                    1.into(),
                    Packet::from(vec![
                        2.into(),
                        Packet::from(vec![
                            3.into(),
                            Packet::from(vec![4.into(), Packet::from(vec![5, 6, 7])]),
                        ]),
                    ]),
                    8.into(),
                    9.into(),
                ]
                .into(),
                vec![
                    1.into(),
                    Packet::from(vec![
                        2.into(),
                        Packet::from(vec![
                            3.into(),
                            Packet::from(vec![4.into(), Packet::from(vec![5, 6, 0])]),
                        ]),
                    ]),
                    8.into(),
                    9.into(),
                ]
                .into(),
            ),
        ]
    }

    #[test]
    fn parsing() {
        let input = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 13);
    }
}
