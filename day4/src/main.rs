extern crate util;
mod interval;
use crate::interval::interval_pair::IntervalPair;
use interval::interval::Interval;
use util::read_file_lines;

fn main() {
    let lines: Vec<String> = read_file_lines();
    let interval_pairs = get_interval_pairs_by_lines(&lines);

    let nbr_contained_intervals = get_number_of_contained_intervals(&interval_pairs);
    println!("Number of contained pairs: {}", nbr_contained_intervals);

    let nbr_overlapping_intervals = get_number_of_overlapping_intervals(&interval_pairs);
    println!("Number of overlapping pairs: {}", nbr_overlapping_intervals);
}

fn get_interval_pairs_by_lines(lines: &Vec<String>) -> Vec<IntervalPair> {
    let mut interval_pairs: Vec<IntervalPair> = Vec::new();
    for line in lines {
        let interval_pair = get_interval_pair_by_line(line);
        interval_pairs.push(interval_pair);
    }
    return interval_pairs;
}

fn get_interval_pair_by_line(line: &str) -> IntervalPair {
    let interval_str_vec: Vec<&str> = line.split(",").collect();
    assert!(interval_str_vec.len() == 2);
    let first_interval_str: &str = interval_str_vec[0];
    let second_interval_str: &str = interval_str_vec[1];
    let first = get_interval_by_string(first_interval_str);
    let second = get_interval_by_string(second_interval_str);
    return IntervalPair { first, second };
}

fn get_interval_by_string(interval_string: &str) -> Interval {
    let interval_indices: Vec<&str> = interval_string.split("-").collect();
    assert!(interval_indices.len() == 2);
    let start: u32 = interval_indices[0].parse().expect("not a number");
    let stop: u32 = interval_indices[1].parse().expect("not a number");
    return Interval { start, stop };
}

fn get_number_of_contained_intervals(intervals: &[IntervalPair]) -> i32 {
    let mut count: i32 = 0;
    for interval_pair in intervals {
        if interval_pair.is_any_contained() {
            count += 1;
        }
    }
    return count;
}

fn get_number_of_overlapping_intervals(intervals: &[IntervalPair]) -> i32 {
    let mut count = 0;
    for interval in intervals {
        if interval.overlapps() {
            count += 1;
        }
    }
    return count;
}
