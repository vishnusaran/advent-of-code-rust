use std::{fs:: File, io::{self, BufRead}};
use std::io::BufReader;

pub fn load_file(file_path: &str) -> Box< dyn  Iterator<Item=String> + '_> {
    
    // fs::read_to_string(file_path).expect("Something went wrong reading the file")
    Box::new(read_lines(file_path).map(|e| e.unwrap()).into_iter())
}

fn read_lines(file_path: &str) -> std::io::Lines<BufReader<File>> {
    let file_result = File::open(file_path);
    file_result.map(|file| io::BufReader::new(file).lines()).expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_load_file(){
        let file_path = "/Users/mac-m3/.alias";
        let mut alias_data = load_file(file_path);
        assert!(alias_data.next().is_some());
        // assert!(!alias_data.is_empty());
    }
}