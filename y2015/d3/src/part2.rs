use std::fs;
#[allow(dead_code, unused, non_snake_case)]
pub fn solution(input_path: String) {
    let lines = fs::read_to_string(input_path).unwrap();
    let mut current_floor_value = 0;
    let mut num_list = 0;

    // TODO: Need to find a functional way to do it
    for (i, c) in lines.chars().enumerate() {
        current_floor_value = match c {
            '(' => current_floor_value + 1,
            ')' => current_floor_value - 1,
            _ => current_floor_value,
        };
        if current_floor_value == -1 {
            num_list = i + 1;
            break;
        }
    }

    println!("Day 3 Part2 solution: {}", num_list);
}
