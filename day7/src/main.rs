use std::{fs, io::Error};
use std::collections::BTreeMap;

fn main() {
    let input = fs::read_to_string("day7/input.txt").unwrap();
    println!("{:?}", one(&input));
    println!("{:?}", two(&input));
}

fn parse_line(line: &str) -> Result<Event, Error> {
    let mut i = line.split_whitespace();
    match i.next() {
        Some("$") => match i.next() {
            Some("ls") => Ok(Event::CmdLs),
            Some("cd") => Ok(Event::CmdCd(i.next().unwrap().to_string())),
            Some(_) => panic!("Unknown cmd"),
            None => panic!("Not a cmd"),
        }
        Some("dir") => Ok(Event::Dir(i.next().unwrap().to_string())),
        Some(size) => {
            let size = size.parse::<usize>().unwrap();
            let name = i.next().unwrap().to_string();
            Ok(Event::File(name, size))
        }
        None => panic!("unable to parse line")
    }
}

fn one(input: &str) -> Result<usize, ()> {
    let dirs = get_dirs(input);
    let sum = dirs.iter()
        .filter(|(_, &s)| s <= 100000)
        .map(|(_,s)| s)
        .sum();
    Ok(sum)
}

fn two(input: &str) -> Result<usize, ()> {
    let dirs = get_dirs(input);
    let used = *dirs.get("/").unwrap();
    let free = 70000000 - used;
    let req = 30000000 - free;
    let sum = dirs.into_iter()
        .filter(|(_, s)| *s > req)
        .map(|(_,s)| s)
        .reduce(|a, i| {
            if a >= i { i } else { a }
        })
        .unwrap();
    Ok(sum)
}

fn get_dirs(input: &str) -> BTreeMap<String, usize> {
    let mut iter = input.lines();
    let mut dirs: BTreeMap<String, usize> = BTreeMap::new();
    let mut cwd: String = String::from("/");
    iter.next();
    for line in iter {
        match parse_line(line) {
            Ok(Event::CmdCd(path)) => {
                if path == ".." {
                    let size = dirs.entry(cwd.clone()).or_insert(0).clone();
                    let mut mid = cwd.rfind('/').unwrap();
                    if mid == 0 {
                        mid = 1;
                    }
                    cwd = cwd.split_at(mid).0.to_string();
                    dirs.entry(cwd.clone()).and_modify(|s| *s += size);
                } else {
                    if cwd != "/" {
                        cwd.push('/');
                    }
                    cwd.push_str(&path);
                }
            },
            Ok(Event::CmdLs) => {
                dirs.entry(cwd.clone()).or_insert(0);
            },
            Ok(Event::Dir(_name)) => continue,
            Ok(Event::File(_name, size)) => {
                dirs.entry(cwd.clone()).and_modify(|s| *s += size).or_insert(size);
            },
            Err(_) => panic!("error"),
        }
    }
    dirs
}

enum Event {
    CmdCd(String),
    CmdLs,
    Dir(String),
    File(String, usize),
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
$ cd ..
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), Ok(95437))
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), Ok(24933642))
    }
}