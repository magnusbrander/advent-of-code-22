extern crate my_util;
use std::{env, fs, collections::HashSet};
use my_util::get_char_intersection;

fn main() {

    let args: Vec<String> = env::args().collect(); 
    let file_path: &String = &args[1];
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let lines_content = get_lines_content(file_content);
    let common_chars_in_lines = get_common_chars_for_lines_content(lines_content);
    let mut priority_sum = 0;
    for common_letters in common_chars_in_lines {
        for common_letter in common_letters {
            let priority = get_priority_by_letter(common_letter);
            priority_sum += priority;
        }
    }
    println!("Sum: {}", priority_sum);
}


struct  LineContent {
    first_chars: Vec<char>,
    second_chars: Vec<char>,
}

fn get_lines_content(file_content: String) -> Vec<LineContent> {
    let mut lines_content: Vec<LineContent> = Vec::new();
    for line in file_content.lines(){
        let line_length = line.len();
        let middle_index = line_length / 2;
        let first_chars: Vec<char> = line[0..middle_index].chars().collect();
        let second_chars: Vec<char> = line[middle_index..].chars().collect();
        let line_content = LineContent {
            first_chars,
            second_chars
        };
        lines_content.push(line_content);
    }
    return  lines_content;
}

fn get_common_chars_for_lines_content(lines_content: Vec<LineContent>) -> Vec<HashSet<char>> {
    let mut common_chars_in_lines:Vec<HashSet<char>> = Vec::new();
    for line_content in lines_content {
        let common_chars_in_line = get_common_chars_for_line_content(line_content);
        common_chars_in_lines.push(common_chars_in_line);
    }
    return common_chars_in_lines;
}


fn get_common_chars_for_line_content(line_content: LineContent) -> HashSet<char> {
    let first_chars_set:HashSet<char> = HashSet::from_iter(line_content.first_chars.iter().cloned());
    let second_chars_set:HashSet<char> = HashSet::from_iter(line_content.second_chars.iter().cloned());
    let intersection: HashSet<char> = first_chars_set.intersection(&second_chars_set).copied().collect();
    return intersection;

}

fn get_intersecting_chars_for_lines_content(lines_content: Vec<LineContent>) -> HashSet<char> {
    let mut lines_chars: Vec<HashSet<char>> = Vec::new();
    for line_content in lines_content {
        let first_chars :HashSet<char> = HashSet::from_iter(line_content.first_chars.iter().cloned()); 
        let second_chars :HashSet<char> = HashSet::from_iter(line_content.first_chars.iter().cloned()); 
        let line_chars: HashSet<char> = first_chars.union(&second_chars).copied().collect();
        lines_chars.push(line_chars);
    } 
    return get_char_intersection(lines_chars);
}

fn get_priority_by_letter(letter: char) -> i32 {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    match letters.iter().position(|&c| c == letter) {
        Some(i) => {
            return i as i32 + 1;
        },
        None => {
            return 0;
        }
    }; 
}