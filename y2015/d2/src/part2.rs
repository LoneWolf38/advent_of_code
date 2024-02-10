use std::fs;
#[allow(dead_code, unused, non_snake_case)]
pub fn solution(input_path: String) {
    let lines = fs::read_to_string(input_path).unwrap();

    println!("Day 2 Part2 solution: {}", lines.len());
}
