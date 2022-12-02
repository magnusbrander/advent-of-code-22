use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");

    let mut calories_per_elf = get_calories_per_elf(&file_content);
    calories_per_elf.sort_by(|a, b| b.cmp(a));
    
    println!("Maximum calories: {}", calories_per_elf[0]);

    let top_three_sum: i32 = calories_per_elf[0..3].iter().sum();
    println!("Top 3 calories sum: {}", top_three_sum);
}

fn get_calories_per_elf(file_content: &String) -> Vec<i32> {
    let mut calories_per_elf: Vec<i32> = Vec::new();
    let mut current_elf_calories = 0;
    for line in file_content.lines() {
        if line.is_empty() {
            calories_per_elf.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            let calories_on_line: i32 = line.parse().unwrap();
            current_elf_calories += calories_on_line;
        }
    }
    return calories_per_elf;
}
