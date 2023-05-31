use std::{collections::HashSet, fs, ops::Add};

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

pub fn get_max<T>(arr: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    assert!(arr.len() != 0);
    let mut max = arr[0];
    for &val in arr {
        if val > max {
            max = val;
        }
    }
    max
}

pub fn get_min<T>(arr: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    assert!(arr.len() != 0);
    let mut min = arr[0];
    for &val in arr {
        if val < min {
            min = val;
        }
    }
    min
}

pub fn sum_elements<T>(elements: &[T], initial_value: T) -> T
where
    T: Copy + Add<T, Output = T>,
{
    let mut sum = initial_value;
    for &element in elements {
        sum = sum + element;
    }
    sum
}

pub fn sort_vec<T: Ord>(vector: &mut [T], descending: bool) {
    if descending {
        vector.sort_by(|a, b| b.cmp(a));
    } else {
        vector.sort_by(|a, b| a.cmp(b));
    }
}

pub fn read_file_into_lines(file_path: &str) -> Vec<String> {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let mut lines: Vec<String> = Vec::new();
    for line in file_content.lines() {
        lines.push(line.to_string());
    }
    lines
}
