use crate::utils::file_loader::{self, load_file};

fn part1(file_path:&str) {
    calculate_total_fishes(file_path, 80);
}

fn calculate_total_fishes(file_path:&str, days:i32) {
    let fish_ages = load_file(file_path).next().unwrap();
    let mut age_arr = [0;9];
    fish_ages.split(',').map(|e| e.parse::<i32>().unwrap()).for_each(|e| age_arr[e as usize] += 1);
    for i in 1..=days {
        let birthed = age_arr[0];
        for j in 1..=8 {
            age_arr[j -1] = age_arr[j];
        }
        age_arr[6] += birthed;
        age_arr[8] = birthed;
        // println!("Day{i}: {:?}",age_arr)
    }
    let total_fishes:i64 = age_arr.iter().sum();
    println!("Total Fishes:{total_fishes}")
    
}

fn part2(file_path:&str) {
    calculate_total_fishes(file_path, 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day6_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day6/input.txt")
    }
    
    #[test]
    fn test_day6_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day6/input.txt")
    }
}