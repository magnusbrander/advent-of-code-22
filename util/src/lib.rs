use std::{collections::HashSet, fs};
pub fn get_char_intersection(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    if sets.len() == 0 {
        return HashSet::new();
    }
    if sets.len() == 1 {
        return sets[0].clone();
    }
    sets.sort_by(|a, b| a.len().cmp(&b.len()));
    let head = &sets[0];
    let tail = &sets[1..];
    let mut intersection: HashSet<char> = HashSet::new();
    for char in head {
        let mut all_sets_have_char = true;
        for set in tail {
            if !set.contains(char) {
                all_sets_have_char = false;
                break;
            }
        }
        if all_sets_have_char {
            intersection.insert(*char);
        }
    }
    return intersection;
}

pub fn read_file_into_lines(file_path: &str) -> Vec<String> {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let mut lines: Vec<String> = Vec::new();
    for line in file_content.lines() {
        lines.push(line.to_string());
    }
    return lines;
}
