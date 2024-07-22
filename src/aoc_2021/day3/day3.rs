use crate::utils::file_loader::load_file;
use core::iter::Iterator;

fn part1(file_path: &str) {
    let input_vec:Vec<String> = load_file(file_path).collect();
    let gamma_vec:Vec<i32> = get_gamma_vec(&input_vec);
    
    let mut gamma:i64 = 0;
    let mut epsilon:i64 = 0;
    for (i, c) in gamma_vec.iter().rev().enumerate() {
        if *c == 1 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1<< i;
        }
    }

    println!("gamma:{:?}, epsilon:{:?}", gamma, epsilon);
    let prod = gamma * epsilon;
    println!("Result:{prod}")

    
}

fn part2(file_path:&str) {
    let o2_rating = calculate_o2_rating(file_path);
    let co2_rating = calculate_co2_rating(file_path);
    println!("o2_rating:{o2_rating}, co2_rating:{co2_rating}");
    let prod = o2_rating * co2_rating;
    println!("prod:{prod}")

}

fn calculate_o2_rating(file_path: &str) -> i64 {
    calculate_rating(file_path, |gamma| if gamma == 0 {'0'} else {'1'})
}

fn calculate_co2_rating(file_path: &str) -> i64 {
    calculate_rating(file_path, |gamma| if gamma == 1 {'0'} else {'1'})
}

fn calculate_rating(file_path: &str, char_selector: impl Fn(i32) -> char) -> i64 {
    let input_vec:Vec<String> = load_file(file_path).collect();
    let mut final_vec = input_vec;
    let mut pos = 0;
    loop {
        if final_vec.len() == 1 {
            break;
        }
        let gamma = get_gamma_at(&final_vec, pos);
        let gamma_char = char_selector(gamma);
        final_vec.retain(|str| (str.as_bytes()[pos] as char) == gamma_char);
        pos +=1;
    }
    convert_to_num(&final_vec[0])
}

fn convert_to_num(str:&String) -> i64 {
    let mut num:i64 = 0;
    for (i,c) in str.chars().rev().enumerate() {
        if c == '1' {
            num |= 1 << i;
        }
    }
    num
}

fn get_gamma_at (input_vec:&Vec<String>, pos:usize)->i32 {
    let mut count_0 = 0;
    let mut count_1 = 0;
    for ele in input_vec {
        match ele.as_bytes()[pos] as char {
            '0' => count_0 += 1,
            _ => count_1 +=1
        }
    }
    
    if count_0 > count_1 {0} else {1}
}

fn get_gamma_vec(input_vec:&Vec<String>) -> Vec<i32> {
    let mut bit_count_vec: Vec<(i32, i32)> = Vec::new();
    for ele in input_vec {
        if bit_count_vec.is_empty() {
            bit_count_vec.append(vec![(0,0);ele.len()].as_mut())
        }
        for char_elem in ele.chars().enumerate() {
            let (i,c) = char_elem;
            match  c {
                '0' => bit_count_vec[i].0 +=1,
                _ =>  bit_count_vec[i].1 += 1
            }
        }
    }
    bit_count_vec.iter().map(|(count_0, count_1)| if count_0 > count_1 {0} else {1}).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day3_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day3/input.txt")
    }

    #[test]
    fn test_day3_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day3/input.txt")
    }
}
