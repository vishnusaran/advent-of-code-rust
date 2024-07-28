use std::fmt::Display;
use std::{collections::HashMap, hash::Hash};
use  std::cmp;
use regex::Regex;
use crate::utils::file_loader::load_file;


#[derive(PartialEq, Eq, Hash,Debug)]

struct Point {
    x:i32,
    y:i32
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"p({},{})",self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Hash,Debug)]
struct Line {
    p1:Point,
    p2:Point
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({}, {})", self.p1, self.p2)
    }
}


impl From<String> for Line {
    fn from(value: String) -> Self {
        let re = Regex::new(r"(\d+),(\d+)\s+->\s+(\d+),(\d+)").unwrap();
        let captures = re.captures(value.as_str()).unwrap();
        let(p1x, p1y, p2x,p2y) = (&captures[1],&captures[2],&captures[3],&captures[4]);
        let left = Point{x:p1x.parse().unwrap(), y:p1y.parse().unwrap()};
        let right = Point{x:p2x.parse().unwrap(), y:p2y.parse().unwrap()};
        Line{p1:left,p2:right}
    }
}


impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.is_horizontal() || self.is_vertial()
    }
    
    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }
    
    fn is_vertial(&self) -> bool {
        self.p1.x == self.p2.x
    }
    
    fn get_points_on_line(&self) -> Box< dyn  Iterator<Item=Point> + '_> {
        if self.is_horizontal() {
            let min = cmp::min(self.p1.x, self.p2.x);
            let max  = cmp::max(self.p1.x, self.p2.x);
            Box::new((min..=max).map(|e| Point{x:e, y:self.p1.y}).into_iter())
        } else if self.is_vertial() {
            let min = cmp::min(self.p1.y, self.p2.y);
            let max  = cmp::max(self.p1.y, self.p2.y);
            Box::new((min..=max).map(|e| Point{x:self.p1.x, y:e}).into_iter())
        } else {
            
            let p1: &Point;
            let p2: &Point;
            if self.p1.x < self.p2.x {
                p1 = &(self.p1);
                p2 = &(self.p2);
            } else {
                p1 = &(self.p2);
                p2 = &(self.p1);
            }
            
            let mut y_step = 1;
            if p1.y > p2.y {
                y_step =  -1;
            }
            let mut y = p1.y;
            
            Box::new((p1.x..=p2.x).map(move |e| {
                let p = Point{x:e, y:y};
                y += y_step;
                p
            }
            
        ).into_iter())
    }
}
}


fn part1(file_path:&str) {
    let mut map:HashMap<Point, i32> = HashMap::new();
    let lines_str = load_file(file_path);
    let lines = lines_str.map(|line| line.into()).filter(|line:&Line| line.is_horizontal_or_vertical()).collect::<Vec<Line>>();
    for line in lines {
        for point in line.get_points_on_line() {
            if let Some(count)  = map.get(&point) {
                map.insert(point, count+1);
            } else {
                map.insert(point, 1);
            }
        }
    }
    let overlaps = map.values().filter(|&&v| v > 1).count();
    println!("overlaps:{overlaps}");
}

fn part2(file_path:&str) {
    let mut map:HashMap<Point, i32> = HashMap::new();
    let lines_str = load_file(file_path);
    let lines = lines_str.map(|line| line.into()).collect::<Vec<Line>>();
    for line in lines {
        for point in line.get_points_on_line() {
            if let Some(count)  = map.get(&point) {
                map.insert(point, count+1);
            } else {
                map.insert(point, 1);
            }
        }
    }
    let overlaps = map.values().filter(|&&v| v > 1).count();
    println!("overlaps:{overlaps}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day5_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day5/input.txt")
    }
    
    #[test]
    fn test_day5_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day5/input.txt")
    }
}