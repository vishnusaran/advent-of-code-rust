use std::collections::{HashMap};

use crate::utils::file_loader::load_file;

fn part1(file_path:&str) {
    let lines = load_file(file_path);
    let char_value = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    let mut sum:i64 = 0;
    for ele in lines {
        if let Some(c) = find_corruption(ele.as_str()) {
            let value = char_value.get(&c).unwrap();
            sum += value;
        }
    }
    println!("{sum}");
}

fn part2(file_path:&str) {
    let char_value = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4)
    ]);

    let lines = load_file(file_path);
    let mut  scores:Vec<i64> = Vec::new();
    for ele in lines.filter(|line| find_corruption(line.as_str()).is_none()) {
        let complete_seq = complete_seq(ele.as_str());
        // let com_string:String = complete_seq.iter().collect();
        // println!("For {ele} completion is {:?}",com_string)
        let map_output = complete_seq.iter().map(|e| char_value.get(e).unwrap());
        let score:i64 = map_output.fold(0, |acc,x| acc*5 +x);
        scores.push(score)
    }
    scores.sort();
    println!("{:?}", &scores);
    let mid = &scores.len()/2;
    let mid_score = &scores[mid];
    println!("mid:{mid_score}");
}

fn complete_seq(line: &str) -> Vec<char> {
    let mut stack:Vec<char>  = Vec::new();
    let bracket_map = HashMap::from([('(',')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    for ele in line.chars() {
        match ele {
            '(' | '[' | '<' | '{' => stack.push(ele),
            _ => {
                    stack.pop();
                }
        }
    }
    let mut rel_vec:Vec<char> = stack.iter().map(|e| bracket_map.get(e).unwrap().clone()).collect();
    rel_vec.reverse();
    rel_vec
}

fn find_corruption(line:&str) -> Option<char> {
    let mut stack:Vec<char>  = Vec::new();
    let bracket_map = HashMap::from([('(',')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    for ele in line.chars() {
        
        match ele {
            '(' | '[' | '<' | '{' => stack.push(ele),
            _ => {
                if let Some(open_bracket) = stack.pop() {
                    let &expected_close_bracket = bracket_map.get(&open_bracket).unwrap();
                    if expected_close_bracket != ele  {
                        return Some(ele)
                    }
                } else {
                    return Some(ele)
                }
                
            }
        }
    }
    None
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day10_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day10/input.txt")
    }
    
    #[test]
    fn test_day10_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day10/input.txt")
    }
}