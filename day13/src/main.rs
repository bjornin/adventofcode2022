use std::fs;
use std::convert::From;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::iter::zip;

fn main() {
    let input = fs::read_to_string("day13/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

#[derive(Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Num(u8),
    Nil,
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Nil => match other {
                Packet::Nil => Equal,
                Packet::Num(_) => Ordering::Less,
                Packet::List(_) => {
                    let mut v = Vec::new();
                    v.push(self.clone());
                    Packet::List(v).cmp(&other)
                }
            }
            Packet::Num(l) => match other {
                Packet::Nil => Ordering::Greater,
                Packet::Num(r) => {
                    l.cmp(r)
                }
                Packet::List(_) => {
                    let mut v = Vec::new();
                    v.push(self.clone());
                    Packet::List(v).cmp(&other)
                }
            }
            Packet::List(l) => match other {
                Packet::Nil => Ordering::Greater,
                Packet::Num(_) => {
                    let mut v = Vec::new();
                    v.push(other.clone());
                    self.cmp(&Packet::List(v))
                }
                Packet::List(r) => {
                    for (a, b) in zip(l, r) {
                        match (*a).cmp(b) {
                            Equal => continue,
                            ret => {
                                return ret
                            }
                        }
                    }
                    l.len().cmp(&r.len())
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let i = self.cmp(other);
        Some(i)
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl Eq for Packet { }

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn is_correct(&self) -> bool {
        self.left < self.right
    }
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let mut packets = value.split_whitespace();
        let mut l_iter = packets.next().unwrap_or("").chars();
        let mut r_iter = packets.next().unwrap_or("").chars();
        let left = parse(&mut l_iter);
        let right = parse(&mut r_iter);

        Pair {
            left,
            right,
        }

    }
}

fn parse<T>(input: &mut T) -> Packet
where
    T: IntoIterator + Iterator<Item = char>
{
    let mut p: Vec<Packet> = Vec::new();
    let mut num: String = String::new();

    while let Some(c) = input.next() {
        match c {
            '[' => p.push(parse(input)), 
            ']' => {
                match num.parse::<u8>() {
                    Ok(n) => {
                        p.push(Packet::Num(n));
                        num.clear();
                    }
                    Err(_) => {
                        if p.len() == 0 {
                            p.push(Packet::Nil)
                        }
                    }
                }
                return Packet::List(p)
            }
            ',' => {
                match num.parse::<u8>() {
                    Ok(n) => {
                        p.push(Packet::Num(n));
                        num.clear();
                    }
                    Err(_) => continue,
                }
            }
            n => num.push(n),
        };
    }
    Packet::List(p)
}

fn one(input: &str) -> usize {
    let pairs: Vec<Pair> = input.split("\n\n").map(Pair::from).collect();
    
    pairs.into_iter().enumerate()
        .filter(|(_, p)| p.is_correct())
        .map(|(i, _)| i + 1).sum()
}

fn two(input: &str) -> usize {
    let mut packets: Vec<Packet> = input.trim()
        .split_whitespace()
        .map(|t| parse(&mut t.chars()))
        .collect();
    let a = parse(&mut "[[2]]".chars());
    let b = parse(&mut "[[6]]".chars());
    packets.push(a.clone());
    packets.push(b.clone());
    packets.sort();

    packets.into_iter().enumerate().filter(|(_, t)| t == &a || t == &b).map(|(i, _)| i + 1).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering::{Greater, Less};

    const INPUT: &str = "\
[1,1,3,1,1]
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
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

const LT: &str = "\
[0]
[1]

[1]
[2]

[1]
[2,1]

[9,1]
[10]

[[2]]
[[3]]

[[2]]
[[2],1]

[[2],2]
[[2],3]

[1,1]
[1,2]

[]
[[]]

[[[]],1]
[[[]],2]
";

const EQ: &str = "\
[]
[]

[[1],2]
[1,[2]]

[1]
[1]

[1,2]
[1,2]

[[2]]
[[2]]

[[],1]
[[],1]

[1,[]]
[1,[]]

[1]
[[[1]]]

[1]
[[1]]

[[1]]
[1]
";

const TT: &str = "\
[[],5]
[[4,7],10]
";

    #[test]
    fn test_tt() {
        let pairs: Vec<Pair> = TT.split("\n\n").map(Pair::from).collect();
        for p in pairs {
            assert_eq!(p.left.cmp(&p.right), Less);
            assert_eq!(p.right.cmp(&p.left), Greater);
        }
    }

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 13);
    }

    #[test]
    fn test_eq() {
        let pairs: Vec<Pair> = EQ.split("\n\n").map(Pair::from).collect();
        for p in pairs {
            assert_eq!(p.left.cmp(&p.right), Equal);
            assert_eq!(p.right.cmp(&p.left), Equal);
        }
    }
    #[test]
    fn test_lt() {

        let pairs: Vec<Pair> = LT.split("\n\n").map(Pair::from).collect();
        for p in pairs {
            assert_eq!(p.left.cmp(&p.right), Less);
        }
    }

    #[test]
    fn test_gt() {
        let pairs: Vec<Pair> = LT.split("\n\n").map(Pair::from).collect();
        for p in pairs {
            assert_eq!(p.right.cmp(&p.left), Greater);
        }
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 140)
    }
}