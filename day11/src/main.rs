#![feature(iter_array_chunks)]

use std::{fs, collections::VecDeque};

fn main() {
    let input = fs::read_to_string("day11/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

struct Monkey {
    items: VecDeque<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    test: u64,
    t_monkey: usize,
    f_monkey: usize,
    inspected: usize,
}

impl Monkey {
    fn new(items: VecDeque<u64>,
        op: Box<dyn Fn(u64) -> u64>,
        test: u64,
        t_monkey: usize,
        f_monkey: usize) -> Monkey
    {
        Monkey {
            items,
            op,
            test,
            t_monkey,
            f_monkey,
            inspected: 0,
        }
    }
}

fn parse_numbers(s: &str) -> VecDeque<u64> {
    s.split_terminator(&[' ', ','][..])
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<VecDeque<u64>>()
}

fn parse_op(s: &str) -> Box::<dyn Fn(u64) -> u64> {
    let group = s.split_whitespace().collect::<Vec<&str>>();
    let num = group[group.len() - 1];
    let op = group[group.len() - 2];
    match op {
        "*" => match num.parse::<u64>() {
            Ok(n) => Box::new(move |a| a * n),
            Err(_) => Box::new(move |a| a * a),
        }
        "+" => match num.parse::<u64>() {
            Ok(n) => Box::new(move |a| a + n),
            Err(_) => panic!("Parse error")
        }
        _ => panic!("Op error"),
    }
}

fn one(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for [_, i, o, t, tm, fm] in input.lines()
        .filter(|&l| !l.trim().is_empty())
        .map(|l| l.trim())
        .array_chunks::<6>() {

        let items = parse_numbers(i);
        let op = parse_op(o);
        let test = parse_numbers(t)[0];
        let true_monkey = parse_numbers(tm)[0] as usize;
        let false_monkey = parse_numbers(fm)[0] as usize;

        monkeys.push(Monkey::new(items, op, test, true_monkey, false_monkey));
    }
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(old) = monkeys[i].items.pop_front() {
                monkeys[i].inspected += 1;
                let mut new = (monkeys[i].op)(old);
                new = new / 3;
                let next_m = if new % monkeys[i].test == 0 {
                    monkeys[i].t_monkey
                } else {
                    monkeys[i].f_monkey
                };
                monkeys[next_m].items.push_back(new);
            }
        }
    }
    monkeys
        .sort_by(|a, b| b.inspected.cmp(&(*a).inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

fn two(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for [_, i, o, t, tm, fm] in input.lines()
        .filter(|&l| !l.trim().is_empty())
        .map(|l| l.trim())
        .array_chunks::<6>() {

        let items = parse_numbers(i);
        let op = parse_op(o);
        let test = parse_numbers(t)[0];
        let true_monkey = parse_numbers(tm)[0] as usize;
        let false_monkey = parse_numbers(fm)[0] as usize;

        monkeys.push(Monkey::new(items, op, test, true_monkey, false_monkey));
    }
    let divider = monkeys.iter().map(|x| x.test).product::<u64>();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(old) = monkeys[i].items.pop_front() {
                monkeys[i].inspected += 1;
                let mut new = (monkeys[i].op)(old);
                new = new % divider;
                let next_m = if new % monkeys[i].test == 0 {
                    monkeys[i].t_monkey
                } else {
                    monkeys[i].f_monkey
                };
                monkeys[next_m].items.push_back(new);
            }
        }
    }
    monkeys
        .sort_by(|a, b| b.inspected.cmp(&(*a).inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
Monkey 0:
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
    If false: throw to monkey 1
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 10605)
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 2713310158)
    }
}