use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::utils::file_loader::load_file;


fn part1(file_path:&str) {
    let lines = load_file(file_path).flat_map(|str| str.split('|').map(str::to_owned).collect::<Vec<_>>());
    let output_iter = lines.enumerate().filter(|(i, _)| i%2 ==1);
    let mut count_1_4_7_8:usize = 0;
    for (_, string) in output_iter {
        let output_digits = string.split(' ').filter(|ele| !ele.is_empty());
        for digit in output_digits {
            let digit_len = digit.len();
            
            match digit_len {
                2 | 3| 4 |7 => count_1_4_7_8 += 1,
                _ => ()
            }
            // println!("digit:{}[{}], count:{}", digit, digit_len, count_1_4_7_8);
        }
    }
    println!("output:{count_1_4_7_8}")
    

}

fn part2(file_path:&str) {
    let mut sum:u64 = 0;
    for line in load_file(file_path) {
        let output = calculate_output(line);
        sum += output;
    }
    println!("Sum:{sum}")
    

}

fn calculate_output(line:String) -> u64 {
    let splits:Vec<&str> = line.split('|').collect();
    let input = splits[0];
    let output = splits[1];
    let mut input_digits:Vec<&str> = Vec::new();
    
    for str in input.split(' ') {
        input_digits.push(str);
    }

    
    let one = input_digits.iter().find(|digit| digit.len() == 2).unwrap();
    let seven = input_digits.iter().find(|digit| digit.len() == 3).unwrap();
    let four = input_digits.iter().find(|digit| digit.len() ==4).unwrap();
    let eight = input_digits.iter().find(|digit| digit.len() == 7).unwrap();
    let six = input_digits.iter().filter(|digit| digit.len() == 6).find(|digit| {
        let seven_set:HashSet<char> = seven.chars().collect();
        let digit_set:HashSet<char> = digit.chars().collect();
        let result_set = seven_set.difference(&digit_set);
        result_set.count()  != 0
    }).unwrap();
    let zero = input_digits.iter().filter(|digit| digit.len() == 6 && digit != &six).find (|digit| {
        let four_set:HashSet<char> = four.chars().collect();
        let digit_set:HashSet<char> = digit.chars().collect();
        four_set.difference(&digit_set).count() != 0

    }).unwrap();
    let nine = input_digits.iter().filter(|digit| digit.len() == 6 && digit != &six && digit != &zero).next().unwrap();
    let three = input_digits.iter().filter(|digit| digit.len() == 5).find(|digit|{
        let seven_set:HashSet<char> = seven.chars().collect();
        let digit_set:HashSet<char> = digit.chars().collect();
        seven_set.difference(&digit_set).count() == 0
    }).unwrap();
    let five = input_digits.iter().filter(|digit| digit.len() == 5 && digit != & three).find(|digit| {
        let six_set:HashSet<char> = six.chars().collect();
        let digit_set:HashSet<char> = digit.chars().collect();
        six_set.intersection(&digit_set).count() == 5
    }).unwrap();

    let two = input_digits.iter().filter(|digit| digit.len() == 5 && digit != & three && digit != &five).next().unwrap();

    let mut str_to_i:HashMap<String,i32> = HashMap::new();
    
    insert_sorted_char_to_map(&mut str_to_i,zero,0);
    insert_sorted_char_to_map(&mut str_to_i,one,1);
    insert_sorted_char_to_map(&mut str_to_i,two,2);
    insert_sorted_char_to_map(&mut str_to_i,three,3);
    insert_sorted_char_to_map(&mut str_to_i,four,4);
    insert_sorted_char_to_map(&mut str_to_i,five,5);
    insert_sorted_char_to_map(&mut str_to_i,six,6);
    insert_sorted_char_to_map(&mut str_to_i,seven,7);
    insert_sorted_char_to_map(&mut str_to_i,eight,8);
    insert_sorted_char_to_map(&mut str_to_i,nine,9);

    let mut result:u64 = 0;
    for digit in output.split(' ').filter(|e| !e.is_empty()) {
        let mut chars = digit.chars().collect::<Vec<char>>();
        chars.sort();
        let str:String = chars.iter().collect();
        let &num = str_to_i.get(&str).unwrap();
        result = result * 10 + num as u64;
    }


    result

}

fn insert_sorted_char_to_map(map: &mut HashMap<String, i32>, str:&str, value:i32) {
    let mut chars = str.chars().collect::<Vec<char>>();
    chars.sort();
    map.insert(chars.iter().collect(), value);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day8_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day8/input.txt")
    }
    
    #[test]
    fn test_day8_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day8/input.txt")
    }
}