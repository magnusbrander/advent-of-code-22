use util;

fn main() {
    let lines = util::read_file_lines();
    let mut calories_per_elf = get_calories_per_elf(&lines);
    util::sort_vec(&mut calories_per_elf, true);
    let max_calories = util::get_max(&calories_per_elf);
    let top_three_sum = util::sum_elements(&calories_per_elf[0..3], 0);

    println!("Max calories: {}, top 3 sum: {}", max_calories, top_three_sum);
}

fn get_calories_per_elf(lines: &Vec<String>) -> Vec<i32> {
    let mut cals_per_elf: Vec<i32> = Vec::new();
    let mut cals_sum = 0;
    for line in lines {
        if line.is_empty() {
            cals_per_elf.push(cals_sum);
            cals_sum = 0;
        } else {
            let cals = line.parse::<i32>().unwrap(); 
            cals_sum = cals_sum + cals;
        }
    }
    cals_per_elf.push(cals_sum);
    cals_per_elf
}
