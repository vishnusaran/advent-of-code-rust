use core::iter::Iterator;
use crate::utils::file_loader::load_file;
use regex::Regex;


#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down
}

#[derive(Debug)]
struct Instruction {
    direction:Direction,
    move_by:u32
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown data.")
        }
    }
}

impl From<&str> for Instruction {
    
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(forward|up|down)\s+(\d+)").unwrap();
        let captures = re.captures(value).unwrap();
        let (inst, mag) = (&captures[1], &captures[2]);
        Instruction{
            direction : inst.into(),
            move_by: mag.parse().unwrap()
        }
        
        
    }
}

fn part1(file_path: &str) {
    let inst:Vec<Instruction> = load_file(file_path).map(|line| line.as_str().into()).collect();
    let mut forward= 0;
    let mut depth = 0;

    for ele in inst {
        match ele.direction   {
            Direction::Forward => forward += ele.move_by,
            Direction::Up => depth -= ele.move_by,
            Direction::Down => depth += ele.move_by
        };
    }

    println!("forward:{forward}");
    println!("depth:{depth}");
    let product = forward * depth;
    println!("product:{product}");
}

fn part2(file_path: &str) {
    let inst:Vec<Instruction> = load_file(file_path).map(|line| line.as_str().into()).collect();

    let mut forward:u64 = 0;
    let mut depth:u64 = 0;
    let mut aim:u64 = 0;

    for ele in inst {
        match ele.direction {
            Direction::Forward => {
                forward += ele.move_by as u64;
                depth += aim * ele.move_by as u64;
            },
            Direction::Up => aim -= ele.move_by as u64,
            Direction::Down => aim += ele.move_by as u64
        }
    }
    println!("forward:{forward}");
    println!("depth:{depth}");
    println!("aim:{aim}");
    let product = forward * depth;
    println!("product:{product}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day2_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day2/input.txt");
    }

    #[test]
    fn test_day2_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day2/input.txt");
    }
}