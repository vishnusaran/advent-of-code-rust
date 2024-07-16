
use core::iter::Iterator;
use crate::utils::file_loader::load_file;

fn part1(file_path:&str) {
    let iter1 = load_file(file_path).map(|e| e.parse::<i32>().unwrap());
    let mut iter2 = load_file(file_path).map(|e| e.parse::<i32>().unwrap());
    iter2.next();
    let zip = iter1.zip(iter2);
    let count = find_increasing_count(zip);

    println!("{count}");    
}

fn find_increasing_count(iter: impl Iterator<Item=(i32,i32)>) -> usize { 
    iter.filter(|(l,r)| r > l).count()
}

fn part2(file_path:&str) {
    let iter1 = load_file(file_path).map(|e| e.parse::<i32>().unwrap());
    let mut iter2 = load_file(file_path).map(|e| e.parse::<i32>().unwrap());
    let mut iter3 = load_file(file_path).map(|e| e.parse::<i32>().unwrap());
    

    iter2.next();
    iter3.next();
    iter3.next();

    let sum_vec:Vec<i32> = iter1.zip(iter2.zip(iter3)).map(|(a,(b,c))| {a+b+c}).collect();
    let sum_iter1 = sum_vec.iter().map(|e| *e);
    let mut sum_iter2 = sum_vec.iter().map(|e| *e);
    sum_iter2.next();
    let zip_iter = sum_iter1.zip(sum_iter2);
    let num_count = find_increasing_count(zip_iter);
    println!("{num_count}");
    

}



#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_day1_part1() {
        part1("/Users/mac-m3/code/advent-of-code-rust/src/aoc_2021/day1/input.txt");
    }
    #[test]
    fn test_day1_part2(){        
        part2("/Users/mac-m3/code/advent-of-code-rust/src/aoc_2021/day1/input.txt");
    }
}