#![feature(let_chains)]

use std::collections::{VecDeque, HashSet};
use std::{fs, fmt};
use std::convert::From;
use regex::Regex;
use std::ops::{Index, IndexMut};

impl<T> Index<Robot> for Vec<T> {
    type Output = T;
    fn index(&self, index: Robot) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Robot> for Vec<T> {
    fn index_mut(&mut self, index: Robot) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Copy, Clone)]
struct RobotBlueprint {
    bot: Robot,
    ore: usize,
    clay: usize,
    obsidian: usize,
}

struct Blueprint {
    id: usize,
    ore: RobotBlueprint,
    clay: RobotBlueprint,
    obsidian: RobotBlueprint,
    geode: RobotBlueprint,
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(\d+)+").unwrap();
        let [id, o_ore, c_ore, ob_ore, ob_clay, g_ore, g_ob]: [usize; 7] = re.find_iter(value)
            .filter_map(|d| d.as_str().parse().ok())
            .collect::<Vec<usize>>().try_into().unwrap();
           
        Blueprint {
            id,
            ore: RobotBlueprint { bot: Robot::Ore, ore: o_ore, clay: 0, obsidian: 0 },
            clay: RobotBlueprint { bot: Robot::Clay, ore: c_ore, clay: 0, obsidian: 0 },
            obsidian: RobotBlueprint { bot: Robot::Obsidian, ore: ob_ore, clay: ob_clay, obsidian: 0 },
            geode: RobotBlueprint { bot: Robot::Geode, ore: g_ore, clay: 0, obsidian: g_ob },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    bots: Vec<usize>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "t: {}", self.time)?;
        writeln!(f, "ore: {}", self.ore)?;
        writeln!(f, "clay: {}", self.clay)?;
        writeln!(f, "obsidian: {}", self.obsidian)?;
        writeln!(f, "geode: {}", self.geode)?;
        writeln!(f, "bot: {:?}", self.bots)?;
        write!(f, "")
    }
}

impl State {
    fn new(time: usize) -> Self {
        State {
            time,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            bots: vec![1,0,0,0]
        }
    }

    fn try_build(&self, r: RobotBlueprint) -> Option<Self> {
        if self.ore >= r.ore && self.clay >= r.clay && self.obsidian >= r.obsidian {
            let mut state = self.gather();
            state.bots[r.bot] += 1;
            return Some(State {
                ore: state.ore - r.ore,
                clay: state.clay - r.clay,
                obsidian: state.obsidian - r.obsidian,
                ..state
            })
        }
        None
    }

    fn gather(&self) -> Self {
        State {
            time: self.time - 1,
            ore: self.ore + self.bots[Robot::Ore],
            clay: self.clay + self.bots[Robot::Clay],
            obsidian: self.obsidian + self.bots[Robot::Obsidian],
            geode: self.geode + self.bots[Robot::Geode],
            ..self.clone()
        }
    }
}

fn main() {
    let input = fs::read_to_string("day19/input.txt").unwrap();
    // println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> usize {
    let blueprints: Vec<Blueprint>  = input.lines().map(Blueprint::from).collect();
    solve(blueprints.as_slice(), 24).iter().enumerate().map(|(i, v)| (i + 1) * v).sum::<usize>()
}
fn two(input: &str) -> usize {
    let blueprints: Vec<Blueprint>  = input.lines().map(Blueprint::from).collect();
    solve(blueprints[..3.min(blueprints.len())].as_ref(), 32).iter().product::<usize>()
}

fn solve(blueprints: &[Blueprint], time: usize) -> Vec<usize> {
    let mut sum_geode = Vec::new();
    for bp in blueprints {
        let mut visited = HashSet::new();
        let mut state = VecDeque::new();
        state.push_back(State::new(time));
        let mut max_geode = 0;
        let mut snr = 0;
        let max_ore = bp.ore.ore.max(bp.clay.ore).max(bp.obsidian.ore).max(bp.geode.ore);
    
        while let Some(s) = state.pop_front() {
            snr += 1;
            if !visited.insert(s.clone()) {
                continue;
            }
            max_geode = max_geode.max(s.geode);
            if s.time == 0 {
                continue;
            }
            if let Some(ns) = s.try_build(bp.geode) {
                state.push_back(ns);
                continue;
            }
            if let Some(ns) = s.try_build(bp.clay) && s.bots[Robot::Clay] < bp.obsidian.clay {
                state.push_back(ns);
            }
            if let Some(ns) = s.try_build(bp.obsidian) {
                state.push_back(ns);
                continue;
            }
            if let Some(ns) = s.try_build(bp.ore) && s.bots[Robot::Ore] < max_ore {
                state.push_back(ns);
            }
            if s.ore <= max_ore {
                state.push_back(s.gather());
            }
        }
        sum_geode.push(max_geode);
        println!("bp:{} it:{} geode:{}", bp.id, snr, max_geode);
    }
    sum_geode
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 33);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 56 * 62);
    }
}