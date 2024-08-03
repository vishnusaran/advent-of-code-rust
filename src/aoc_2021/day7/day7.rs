use crate::utils::file_loader::load_file;

fn part1(file_path:&str) {
    let sub_positions_str = load_file(file_path).next().unwrap();
    let sub_positions = sub_positions_str.split(',').map(|e| e.parse().unwrap()).collect::<Vec<i32>>();
    let &min = sub_positions.iter().min().unwrap();
    let &max = sub_positions.iter().max().unwrap();

    let mut min_cost = std::u64::MAX;
    

    for exp_pos in min..=max  {
        let fuel_spent = sub_positions.iter().map(|&pos| i32::abs( exp_pos - pos) as u64).sum::<u64>();
        min_cost = min_cost.min(fuel_spent);
    }

    println!("{min_cost}")
    
}

fn part2(file_path:&str) {
    let sub_positions_str = load_file(file_path).next().unwrap();
    let sub_positions = sub_positions_str.split(',').map(|e| e.parse().unwrap()).collect::<Vec<i32>>();
    let &min = sub_positions.iter().min().unwrap();
    let &max = sub_positions.iter().max().unwrap();

    let mut min_cost = std::u64::MAX;
    

    for exp_pos in min..=max  {
        let fuel_spent = sub_positions.iter().map(|&pos| i32::abs( exp_pos - pos) as u64).map(|i| (i * (i+1))/2).sum();
        min_cost = min_cost.min(fuel_spent);
    }

    println!("{min_cost}")
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day7_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day7/input.txt")
    }
    
    #[test]
    fn test_day7_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day7/input.txt")
    }
}