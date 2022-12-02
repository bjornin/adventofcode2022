use std::fs;

fn main() {
    let input = fs::read_to_string("day1/input.txt").unwrap();
    let res = parse(&input);
    println!("{}", res.max());
    println!("{}", res.sum());
}

pub struct Ring {
    buffer: [usize; 3]
}

impl Ring {
    pub fn new() -> Ring {
        let buffer: [usize; 3] = [0; 3];
        Ring {
            buffer,
        }
    }

    pub fn push(&mut self, e: usize) {
        let (index, min) = self.buffer.iter().enumerate().min_by(|(_,x), (_,y)| x.cmp(y)).unwrap();
        if e > *min {
            self.buffer[index] = e;
        }
    }

    pub fn max(&self) -> &usize {
        self.buffer.iter().max().unwrap_or(&0)
    }

    pub fn sum(&self) -> usize {
        self.buffer.iter().sum()
    }
}

fn parse(input: &str) -> Ring {
    let mut buf = Ring::new();
    let mut elf_cal: usize = 0;
    for line in input.lines() {
        match line.parse::<usize>() {
            Ok(num) => elf_cal += num,
            Err(_) => {
                buf.push(elf_cal);
                elf_cal = 0;
                continue;
            }
        }
    }
    buf.push(elf_cal);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    
    #[test]
    fn elf_most_cals() {
        assert_eq!(*parse(INPUT).max(),24000);
    }

    #[test]
    fn three_elfs_most_cals() {
        assert_eq!(parse(INPUT).sum(),45000);
    }
}