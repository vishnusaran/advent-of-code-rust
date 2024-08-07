use std::collections::{HashSet, VecDeque};

use crate::utils::file_loader::load_file;

fn part1(file_path:&str) {
    let mut oct_vec = create_2d_vector(file_path);
    let mut flashes = 0;
    for i in 0..100 {
        flashes += step(&mut oct_vec);
        println!("step:{}", i+1);
        let oct_vec_slice = &oct_vec.as_slice()[1..oct_vec.len()-1];
        print_oct_vec(oct_vec_slice);
        print!("");
    }
    
    println!("{flashes}")    
}

fn print_oct_vec(oct_vec_slice: &[Vec<u8>]){
    for ele in oct_vec_slice {
        let ele_slice = &ele.as_slice()[1..ele.len()-1];
        let line:String = ele_slice.iter().fold("".into(), |mut acc,e| {
            acc.push_str(e.to_string().as_str());
            acc
        });
        println!("{line}");
    }
}

fn part2(file_path:&str) {
    let mut oct_vec = create_2d_vector(file_path);
    let expected_flashes  =100;
    let mut step_count = 0;
    loop {
        step_count+=1;
        println!("step:{step_count}");
        let flashes = step(&mut oct_vec);
        if flashes == expected_flashes {
            break;
        }
    }

    println!("All Flashed:{step_count}");
}

fn step(oct_vec: &mut Vec<Vec<u8>>) -> i32 {
    let mut queue:VecDeque<(usize,usize)> = VecDeque::new();
    for i in 1..oct_vec.len()-1 {
        for j in 1..oct_vec[i].len()-1 {
            oct_vec[i][j] +=1;
            if oct_vec[i][j] > 9 {
                queue.push_back((i,j))
            }
        }
    }
    
    let mut flashed_pos:HashSet<(usize,usize)> = HashSet::new();
    while let Some(pos) = queue.pop_front() {
        if flashed_pos.contains(&pos) {
            continue;
        }
        flashed_pos.insert(pos);
        for (i,j) in get_neighbor(&pos) {
            if oct_vec[i][j] != u8::MAX && !flashed_pos.contains(&(i,j)) {
                oct_vec[i][j] +=1;
                if oct_vec[i][j]> 9 {
                    queue.push_back((i,j))
                }
            }
        }
    }
    for &(i,j) in &flashed_pos {
        oct_vec[i][j] = 0;
    }
    flashed_pos.len() as i32
}

fn get_neighbor(pos:&(usize, usize)) -> [(usize,usize);8] {
    let &(i,j) = pos;
    [(i-1, j-1),(i-1,j),(i-1,j+1),
     (i,j-1),(i,j+1),
     (i+1,j-1),(i+1,j), (i+1,j+1)]
}

fn create_2d_vector(file_path:&str) -> Vec<Vec<u8>> {
    let mut v:Vec<Vec<u8>> = Vec::new();
    for line in load_file(file_path) {
        v.push(line.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>()); 
    }
    let mut v2:Vec<Vec<u8>> = Vec::new();
    let len = &v[0].len() + 2;
    v2.push(vec![u8::MAX;len]);
    for mut ele in v {
        let mut inner_v:Vec<u8> = Vec::new();
        inner_v.push(u8::MAX);
        inner_v.append(&mut ele);
        inner_v.push(u8::MAX);
        v2.push(inner_v);
    }
    v2.push(vec![u8::MAX;len]);
    v2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day11_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day11/input.txt")
    }
    
    #[test]
    fn test_day11_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day11/input.txt")
    }
}