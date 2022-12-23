use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn solve(monkeys: &HashMap<&str, &str>, name: &str) -> i64 {
    let m = monkeys[name];

    let result = match m.split(' ').collect_tuple() {
        Some((m1, "+", m2)) => solve(monkeys, m1) + solve(monkeys, m2),
        Some((m1, "-", m2)) => solve(monkeys, m1) - solve(monkeys, m2),
        Some((m1, "*", m2)) => solve(monkeys, m1) * solve(monkeys, m2),
        Some((m1, "/", m2)) => solve(monkeys, m1) / solve(monkeys, m2),
        _ => m.parse().unwrap(),
    };
    result
}
fn main() {
    let input = fs::read_to_string("day21/input.txt").unwrap();
    println!("{}", one(&input));
    // println!("{}", two(&input));
}

fn one(input: &str) -> i64 {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let (name, expr) = line.split_once(": ").unwrap();
        monkeys.insert(name, expr);
    }
    solve(&monkeys, "root")
}

fn two(input: &str) -> i64 {

}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 152);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 301);
    }
}