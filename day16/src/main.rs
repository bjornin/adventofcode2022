use std::fs;
use std::collections::{HashMap, HashSet, BinaryHeap};
use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug)]
struct Valve<'a> {
    pressure: i32,
    next: Vec<&'a str>
}

impl<'a> Valve<'a> {
    fn new() -> Self {
        Valve {
            pressure: 0,
            next: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Hash)]
struct State<'a> {
    valve: Vec<&'a str>,
    total: i32,
    time: i32,
    opened: Vec<&'a str>
}

impl<'a> State<'a> {
    fn new(valve: Vec<&'a str>, time: i32) -> Self {
        State {
            valve,
            total: 0,
            time,
            opened: Vec::new(),
        }
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> Eq for State<'a> {}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

fn parse(input: &str) -> HashMap<&str, Valve> {
    let valves_pattern = Regex::new(r"[A-Z]{2}").unwrap();
    let pressure_pattern = Regex::new(r"\d+").unwrap();
    let mut valves: HashMap<&str, Valve> = HashMap::new();
    for line in input.lines() {
        let mut v_cap = valves_pattern.captures_iter(line);
        let name = v_cap.next().unwrap().get(0).unwrap().as_str();
        let mut valve = Valve::new();
        for v in v_cap {
            valve.next.push(&v.get(0).unwrap().as_str());
        }
        if let Some(p_cap) = pressure_pattern.captures(line) {
            valve.pressure = p_cap.get(0).unwrap().as_str().parse().unwrap();
        }
        valves.insert(name, valve);
    }
    valves
}

fn main() {
    let input = fs::read_to_string("day16/input.txt").unwrap();
    // println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> i32 {
    let valves = parse(input);

    let mut visited = HashSet::new();
    let mut states: BinaryHeap<State> = BinaryHeap::new();
    
    states.push(State::new(vec!["AA"], 30));
    let mut max_pressure = 0;

    let mut snr = 0;
    while let Some(s) = states.pop() {
        snr += 1;
        if !visited.insert(s.clone()) {
            continue;
        }
        max_pressure = max_pressure.max(s.total);
        if s.time == 0 {
            continue;
        }
        let current = valves.get(s.valve[0]).unwrap();
        if !s.opened.contains(&s.valve[0]) && current.pressure > 0 {
            let mut ns = State {
                time: s.time - 1,
                ..s.clone()
            };
            ns.opened.push(s.valve[0]);
            ns.total += current.pressure * ns.time;
            if !visited.contains(&ns) {
                states.push(ns);
            }
        }
        for n in &current.next {
            let ns = State {
                valve: vec![n],
                time: s.time - 1,
                ..s.clone()
            };
            let bs = State {
                valve: vec![n],
                time: s.time + 1,
                ..s.clone()
            };
            if !visited.contains(&ns) && !visited.contains(&bs)  {
                states.push(ns);
            }
        }
    }
    println!("snr {} vis {}", snr, visited.len());
    max_pressure
}

fn two(input: &str) -> i32 {
    let valves = parse(input);

    let mut visited: HashSet<State> = HashSet::new();
    let mut states: BinaryHeap<State> = BinaryHeap::new();
    
    states.push(State::new(vec!["AA","AA"], 26));
    let mut max_pressure = 0;

    let mut snr = 0;
    while let Some(s) = states.pop() {
        snr += 1;
        if !visited.insert(s.clone()) {
            continue;
        }
        
        let mp = max_pressure.max(s.total);
        if mp > max_pressure {
            println!("{:?}", s);
            max_pressure = mp;
        }

        if s.time == 0 {
            continue;
        }

        let pressure_potential: i32 = valves
        .keys()
        .filter(|&a| !s.opened.contains(a))
        .map(|a| valves.get(a).unwrap().pressure)
        .sum();
        
        if max_pressure > s.total + pressure_potential * (s.time - 1) {
            continue;
        }

        // max_pressure = max_pressure.max(s.total);

        for i in 0..2 {
            let v1 = s.valve[i];
            let v2 = s.valve[(i + 1) % 2];
            let current = valves.get(v1).unwrap();
            let other = valves.get(v2).unwrap();
            if !s.opened.contains(&v1) && current.pressure > 0 {
                for next in &other.next {
                    let mut ns = State {
                        valve: vec![v1, next],
                        time: s.time - 1,
                        ..s.clone()
                    };
                    ns.opened.push(v1);
                    ns.total += current.pressure * ns.time;
                    if !visited.contains(&ns) {
                        states.push(ns);
                    }
                }
            }
        }

        let mut perm = Vec::new();
        for v1 in &valves.get(s.valve[0]).unwrap().next {
            for v2 in &valves.get(s.valve[1]).unwrap().next {
                let mut t = vec![*v1, *v2];
                t.sort();
                perm.push(t);
            }
        }
        perm.sort();
        perm.dedup();
        for n in perm {
            let ns = State {
                valve: n.clone(),
                time: s.time - 1,
                ..s.clone()
            };
            let bs = State {
                valve: n.clone(),
                time: s.time + 1,
                ..s.clone()
            };
            if !visited.contains(&ns) && !visited.contains(&bs) {
                states.push(ns);
            }
        }
    }
    println!("snr {} vis {}", snr, visited.len());
    max_pressure
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 1651)
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 1707)
    }
}