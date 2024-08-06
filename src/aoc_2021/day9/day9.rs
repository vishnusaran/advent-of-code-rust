use std::{collections::{HashSet, VecDeque}, u8};

use crate::utils::file_loader::load_file;


fn part1(file_path:&str) {
    // let vector = create_2d_vector(file_path);
    // let mut min_vec:Vec<&u8> = Vec::new();
    
    // for i in 1..&vector.len()-1 {
        
    //     for j in 1..&vector[i].len()-1 {
    //         let ele = &vector[i][j];
    //         if ele < &vector[i-1][j] && ele < &vector[i+1][j] && ele < &vector[i][j-1] && ele < &vector[i][j+1] {
    //             min_vec.push(ele);
    //         }
    //     }
    // }
    // println!("{:?}", min_vec);
    // let sum = min_vec.iter().map(|&&e| e as u32 +1).sum::<u32>();
    // println!("{sum}");

    let height_map = create_2d_vector(file_path);
    let low_points = get_low_points(&height_map);
    println!("{:?}", &low_points);
    let sum = &low_points.iter().map(|&(i,j)| (&height_map)[i][j] as u32 + 1 ).sum::<u32>();
    println!("{sum}")
}

fn part2(file_path:&str) {
    let height_map = create_2d_vector(file_path);
    let low_points = get_low_points(&height_map);
    let mut queue:VecDeque<(usize,usize)> = VecDeque::new();
    
    let mut basin_sizes:Vec<i64> = Vec::new();

    for &low_point in &low_points {
        let height_map_ref = &height_map;
        let mut basin_size:i64 =    0;
        queue.push_back(low_point);
        let mut visited:HashSet<(usize,usize)> = HashSet::new();
        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos){
                continue;
            }
            visited.insert(pos);
            basin_size += 1;
            let (i,j) = pos;
            let value = height_map_ref[i][j] +1 ;
            if height_map_ref[i-1][j] < 9 {
                queue.push_back((i-1,j));
            }
            if height_map_ref[i+1][j] < 9 {
                queue.push_back((i+1, j))
            } 
            if  height_map_ref[i][j-1] < 9 {
                queue.push_back((i,j-1))
            }
            if height_map_ref[i][j+1] < 9 {
                queue.push_back((i,j+1))
            }
        }        

        basin_sizes.push(basin_size);
        
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    let product = basin_sizes.iter().take(3).product::<i64>();
    println!("{product}");
    println!("{:?}", basin_sizes);
}

fn get_low_points(height_map: & Vec<Vec<u8>>) -> Vec<(usize,usize)> {
    let mut min_points:Vec<(usize,usize)> = Vec::new();
    for i in 1..height_map.len() -1 {
        for j in 1..height_map[i].len() -1 {
            let ele = height_map[i][j];
            if ele < height_map[i-1][j] && ele < height_map[i+1][j] && ele < height_map[i][j-1] && ele < height_map[i][j+1] {
                min_points.push((i,j))
            }
        }
    }
    min_points
}


fn create_2d_vector(file_path:&str) -> Vec<Vec<u8>> {
    let mut v:Vec<Vec<u8>> = Vec::new();
    for line in load_file(file_path) {
        v.push(line.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>()); 
    }
    let mut v2:Vec<Vec<u8>> = Vec::new();
    let len = &v[0].len() + 1;
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
    fn test_day9_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day9/input.txt")
    }
    
    #[test]
    fn test_day9_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day9/input.txt")
    }
}