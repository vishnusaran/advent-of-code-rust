use std::{borrow::BorrowMut, collections::HashSet, fmt::{write, Display}};
use core::iter::Iterator;


use crate::utils::file_loader::{self, load_file};

#[derive(Debug)]
struct GameBoard {
    id:i32,
    board:Vec<Vec<i32>>,
    row_sets:Vec<HashSet<i32>>,
    col_sets:Vec<HashSet<i32>>,
    run_set:HashSet<i32>
}

impl Display for GameBoard{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strs:Vec<String>  = Vec::new();
        strs.push(format!("id:{}", self.id));
        strs.push("Board:".to_string());
        for ele in &self.board {
            let ele_strs:Vec<String> = ele.iter().map(|e| if self.run_set.contains(e) {format!("#{}", e.to_string())} else {e.to_string()}).collect();
            strs.push(format!("{:?}", ele_strs));
        }

        strs.push("Row Sets:".to_string());
        for (i, set)in self.row_sets.iter().enumerate() {
            strs.push(format!("{}:{:?}", i,set))
        }

        strs.push("Col Sets:".to_string());
        for (i, set)in self.col_sets.iter().enumerate() {
            strs.push(format!("{}:{:?}", i,set))
        }
        strs.push(format!("Run Set:{:?}", &self.run_set));


        let mut result:std::fmt::Result = Ok(());
        for ele in &strs {
            result = result.and_then(|_| writeln!(f, "{}", ele));
        }
        result
        
    }
}



impl GameBoard {
    fn new<'a, I> (iter:& mut I, id:i32) -> GameBoard
    where I: Iterator<Item = String> +'a {
        let mut row_sets:Vec<HashSet<i32>> = Vec::new();
        let mut col_sets:Vec<HashSet<i32>> = Vec::new();
        let mut board:Vec<Vec<i32>> = Vec::new();
        for ele in iter.take(5) {
            let splits:Vec<i32> = ele.split(' ').filter(|&e| !e.is_empty()).map(|e| e.parse::<i32>().unwrap()).collect();
            if col_sets.is_empty() {
                for _ in 0..splits.len() {
                    col_sets.push(HashSet::new());
                }
            }

            let mut row:HashSet<i32> = HashSet::new();
            for (i, num) in splits.iter().enumerate() {
                row.insert(*num);
                col_sets[i].insert(*num);
            }
            row_sets.push(row);
            board.push(splits);
        }
        Self {id:id, board: board, row_sets:row_sets , col_sets: col_sets, run_set:HashSet::new() }
    }

    fn add_run(&mut self, run:i32) {
        &mut self.run_set.insert(run);
        
        for mut ele in &mut self.row_sets {
            ele.remove(&run);
        }

         for mut ele in &mut self.col_sets {
            ele.remove(&run);
        }
    }   

    fn bingo(&self) -> bool {
        self.row_sets.iter().find(|e| e.is_empty()).is_some() || 
        self.col_sets.iter().find(|e| e.is_empty()).is_some()
    }

    fn board_sum(&self) ->i32 {
        self.row_sets.iter().map(|hs| {
            hs.iter().filter(|e| !self.run_set.contains(e)).sum::<i32>()
        }).sum::<i32>()
    }
}


fn part1(file_path : &str) {
    let mut iter = load_file(file_path);
    let first_line = iter.next();
    let run:Vec<i32> = first_line.unwrap().split(',').filter(|e| !e.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect();
    let mut boards:Vec<GameBoard> = Vec::new();
    
    while iter.next().is_some() {
        let board = GameBoard::new(& mut iter, boards.len() as i32);
        boards.push(board);
        
    }

    
    let mut rel:i32 = -1;

    for ele in run {
        for board in &mut boards {
            println!("Adding run:{} to board:{}", &ele, &board.id);
            board.add_run(ele);
            // println!("{}", board);
            if board.bingo() {
                let sum = board.board_sum();
                let id = board.id;
                println!("boardId:{id}");
                println!("sum:{sum}");
                println!("ele:{ele}");
                rel =  sum * ele;
                break;
            }
        }
        if rel >=0 {
            break;
        }
    }

    println!("Result:{rel}");

}

fn part2(file_path : &str) {
    let mut iter = load_file(file_path);
    let first_line = iter.next();
    let run:Vec<i32> = first_line.unwrap().split(',').filter(|e| !e.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect();
    let mut boards:Vec<GameBoard> = Vec::new();
    let mut unfinished_boards:HashSet<i32> = HashSet::new();
    
    while iter.next().is_some() {
        let board = GameBoard::new(& mut iter, boards.len() as i32);
        unfinished_boards.insert(boards.len() as i32);
        boards.push(board);
        
    }
    
    for ele in run {
        for board in &mut boards {
            println!("Adding run:{} to board:{}", &ele, &board.id);
            if unfinished_boards.contains(&board.id) {
                board.add_run(ele);
                if board.bingo() {
                    if(unfinished_boards.len() == 1 ){
                        let sum = board.board_sum();
                        let id = board.id;
                        println!("boardId:{id}");
                        println!("sum:{sum}");
                        println!("ele:{ele}");
                        let rel =  sum * ele;
                        println!("result: {rel}");
                    }
                    unfinished_boards.remove(&board.id);
                }
            }
            if unfinished_boards.is_empty() {
                break;
            }
        }
        if unfinished_boards.is_empty() {
            break;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day4_part1() {
        part1("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day4/input.txt")
    }

    #[test]
    fn test_day4_part2() {
        part2("/Users/madvish/code/advent-of-code-rust/src/aoc_2021/day4/input.txt")
    }
}
