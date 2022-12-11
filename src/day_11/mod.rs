mod parsing;

use std::sync::mpsc::{Receiver, Sender};

use crate::{ParseError, SolveError};

pub struct Monkey {
    items_in: Sender<usize>,
    items_out: Receiver<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    true_monkey: usize,
    false_monkey: usize,
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Monkey>;
    type Output = usize;
    const DAY: u8 = 11;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        input
            .split("\n\n")
            .map(|monkey| parsing::parse_monkey(monkey))
            .collect::<Result<_, _>>()
    }

    fn part_1(monkeys: Self::Input) -> Result<Self::Output, SolveError> {
        let mut inspection_counts = Vec::with_capacity(monkeys.len());

        for _ in 0..monkeys.len() {
            inspection_counts.push(0);
        }

        for _ in 0..20 {
            for (index, monkey) in monkeys.iter().enumerate() {
                for mut item in monkey.items_out.try_iter() {
                    inspection_counts[index] += 1;

                    item = (monkey.operation)(item);
                    item = item / 3;

                    if (monkey.test)(item) {
                        let _ = monkeys[monkey.true_monkey].items_in.send(item);
                    } else {
                        let _ = monkeys[monkey.false_monkey].items_in.send(item);
                    }
                }
            }
        }

        inspection_counts.sort_unstable();

        Ok(inspection_counts[inspection_counts.len() - 2]
            * inspection_counts[inspection_counts.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::Solver;

    use super::*;

    fn get_input() -> Vec<Monkey> {
        let (t0, r0) = channel::<usize>();
        let (t1, r1) = channel::<usize>();
        let (t2, r2) = channel::<usize>();
        let (t3, r3) = channel::<usize>();

        let monkeys = vec![
            Monkey {
                items_in: t0,
                items_out: r0,
                operation: Box::new(|old| old * 19),
                test: Box::new(|item| item % 23 == 0),
                true_monkey: 2,
                false_monkey: 3,
            },
            Monkey {
                items_in: t1,
                items_out: r1,
                operation: Box::new(|old| old + 6),
                test: Box::new(|item| item % 19 == 0),
                true_monkey: 2,
                false_monkey: 0,
            },
            Monkey {
                items_in: t2,
                items_out: r2,
                operation: Box::new(|old| old * old),
                test: Box::new(|item| item % 13 == 0),
                true_monkey: 1,
                false_monkey: 3,
            },
            Monkey {
                items_in: t3,
                items_out: r3,
                operation: Box::new(|old| old + 3),
                test: Box::new(|item| item % 17 == 0),
                true_monkey: 0,
                false_monkey: 1,
            },
        ];

        let _ = monkeys[0].items_in.send(79);
        let _ = monkeys[0].items_in.send(98);

        let _ = monkeys[1].items_in.send(54);
        let _ = monkeys[1].items_in.send(65);
        let _ = monkeys[1].items_in.send(75);
        let _ = monkeys[1].items_in.send(74);

        let _ = monkeys[2].items_in.send(79);
        let _ = monkeys[2].items_in.send(60);
        let _ = monkeys[2].items_in.send(97);

        let _ = monkeys[3].items_in.send(74);

        monkeys
    }

    #[test]
    fn parsing() {
        let input = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let mut monkeys = super::Solver::parse(String::from(input)).unwrap();

        let monkey3 = monkeys.pop().unwrap();
        let monkey2 = monkeys.pop().unwrap();
        let monkey1 = monkeys.pop().unwrap();
        let monkey0 = monkeys.pop().unwrap();

        let items0 = vec![79, 98];
        let items1 = vec![54, 65, 75, 74];
        let items2 = vec![79, 60, 97];
        let items3 = vec![74];

        assert_eq!(monkey0.items_out.try_iter().collect::<Vec<_>>(), items0);
        assert_eq!((monkey0.operation)(79), 1501);
        assert!((monkey0.test)(46));
        assert_eq!(monkey0.true_monkey, 2);
        assert_eq!(monkey0.false_monkey, 3);

        assert_eq!(monkey1.items_out.try_iter().collect::<Vec<_>>(), items1);
        assert_eq!((monkey1.operation)(54), 60);
        assert!((monkey1.test)(38));
        assert_eq!(monkey1.true_monkey, 2);
        assert_eq!(monkey1.false_monkey, 0);

        assert_eq!(monkey2.items_out.try_iter().collect::<Vec<_>>(), items2,);
        assert_eq!((monkey2.operation)(79), 6241);
        assert!((monkey2.test)(26));
        assert_eq!(monkey2.true_monkey, 1);
        assert_eq!(monkey2.false_monkey, 3);

        assert_eq!(monkey3.items_out.try_iter().collect::<Vec<_>>(), items3);
        assert_eq!((monkey3.operation)(74), 77);
        assert!((monkey3.test)(34));
        assert_eq!(monkey3.true_monkey, 0);
        assert_eq!(monkey3.false_monkey, 1);
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 10605);
    }
}
