use std::fs;
#[allow(dead_code, unused, non_snake_case)]
pub fn solution(input_path: String) {
    let input = fs::read_to_string(input_path).unwrap();
    let mut min_number = 0;

    for i in 1.. {
        let hash = md5::compute(format!("{}{}", input.trim(), i).as_bytes());
        let hash_str = format! {"{:x}", hash}.to_string();
        if hash_str.starts_with("000000") {
            min_number = i;
            break;
        }
    }
    println!("Day 4 Part2 solution: {:?}", min_number);
}
