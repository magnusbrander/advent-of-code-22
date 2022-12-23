use std::{env, time::Instant, collections::HashSet};
use util::read_file_into_lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let lines_vec = read_file_into_lines(file_path);
    let dna = lines_vec[0].clone();
    
    let dna_chars = HashSet::from(['a', 't','g','c']);
    
    let now = Instant::now();
    let isdna1 = is_dna1(&dna);
    let t1 = now.elapsed().as_micros();
    println!("{} {}", t1, isdna1);
    let now2 = Instant::now();
    let isdna2 = is_dna2(&dna);
    let t2 = now2.elapsed().as_micros();
    println!("{} {}", t2, isdna2);
    let now3 = Instant::now();
    let isdna3 = is_dna3(&dna, &dna_chars);
    let t3 = now3.elapsed().as_micros();
    println!("{} {}", t3, isdna3);
    println!("Done!");
}

fn is_dna1(dna: &str) -> bool {
    return dna
    .chars()
    .all(|c| c == 'a' || c == 't' || c == 'g' || c == 'c');
}

fn is_dna2(dna: &str) -> bool {
    for c in dna.chars() {
        if c != 'a' && c != 't' && c != 'g' && c != 'c' {
            return false;
        }
    }
    return true;
}


fn is_dna3(dna: &str, dna_chars: &HashSet<char>) -> bool {
    for c in dna.chars() {
        if !dna_chars.contains(&c) {
            return false;
        }
    }
    return true;
}


// extern crate util;
// use util::get_char_intersection;
// use std::{collections::HashSet, env, fs};

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let file_path: &String = &args[1];
//     let file_content = fs::read_to_string(file_path).expect("Failed to read file");
//     let lines_content = get_lines_content(&file_content);
//     let common_chars_in_lines = get_common_chars_for_lines_content(&lines_content);
//     let mut priority_sum_1 = 0;
//     for common_letters in common_chars_in_lines {
//         priority_sum_1 += get_priority_by_letters(&common_letters);
//     }
//     let mut priority_sum_2 = 0;
//     for lines_chunk in lines_content.chunks(3) {
//         let intersection_chars = get_intersecting_chars_for_lines_content(lines_chunk);
//         priority_sum_2 += get_priority_by_letters(&intersection_chars);
//     }
//     println!("Sum: {}", priority_sum_1);
//     println!("Sum: {}", priority_sum_2);
// }

// struct LineContent {
//     first_chars: Vec<char>,
//     second_chars: Vec<char>,
// }

// fn get_lines_content(file_content: &str) -> Vec<LineContent> {
//     let mut lines_content: Vec<LineContent> = Vec::new();
//     for line in file_content.lines() {
//         let line_length = line.len();
//         let middle_index = line_length / 2;
//         let first_chars: Vec<char> = line[0..middle_index].chars().collect();
//         let second_chars: Vec<char> = line[middle_index..].chars().collect();
//         let line_content = LineContent {
//             first_chars,
//             second_chars,
//         };
//         lines_content.push(line_content);
//     }
//     return lines_content;
// }

// fn get_common_chars_for_lines_content(lines_content: &[LineContent]) -> Vec<HashSet<char>> {
//     let mut common_chars_in_lines: Vec<HashSet<char>> = Vec::new();
//     for line_content in lines_content {
//         let common_chars_in_line = get_common_chars_for_line_content(&line_content);
//         common_chars_in_lines.push(common_chars_in_line);
//     }
//     return common_chars_in_lines;
// }

// fn get_common_chars_for_line_content(line_content: &LineContent) -> HashSet<char> {
//     let first_chars_set: HashSet<char> =
//         HashSet::from_iter(line_content.first_chars.iter().cloned());
//     let second_chars_set: HashSet<char> =
//         HashSet::from_iter(line_content.second_chars.iter().cloned());
//     let intersection: HashSet<char> = first_chars_set
//         .intersection(&second_chars_set)
//         .copied()
//         .collect();
//     return intersection;
// }

// fn get_intersecting_chars_for_lines_content(lines_content: &[LineContent]) -> HashSet<char> {
//     let mut lines_chars: Vec<HashSet<char>> = Vec::new();
//     for line_content in lines_content {
//         let first_chars: HashSet<char> =
//             HashSet::from_iter(line_content.first_chars.iter().cloned());
//         let second_chars: HashSet<char> =
//             HashSet::from_iter(line_content.second_chars.iter().cloned());
//         let line_chars: HashSet<char> = first_chars.union(&second_chars).copied().collect();
//         lines_chars.push(line_chars);
//     }
//     return get_char_intersection(lines_chars);
// }

// fn get_priority_by_letter(letter: &char) -> i32 {
//     let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
//         .chars()
//         .collect();
//     match letters.iter().position(|&c| c == *letter) {
//         Some(i) => {
//             return i as i32 + 1;
//         }
//         None => {
//             return 0;
//         }
//     };
// }

// fn get_priority_by_letters(letters: &HashSet<char>) -> i32 {
//     let mut sum = 0;
//     for letter in letters {
//         sum += get_priority_by_letter(&letter);
//     }
//     return sum;
// }
