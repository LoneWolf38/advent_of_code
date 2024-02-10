use std::fs;
#[allow(dead_code, unused, non_snake_case)]
pub fn solution(input_path: String) {
    let lines = fs::read_to_string(input_path).unwrap();
    let up = lines.chars().filter(|&c| c == '(').count();
    let down = lines.chars().filter(|&c| c == ')').count();
    let num_list = up - down;

    // this is also one way to find out the soln
    // let num_list = lines.chars().fold(current_position, |acc, c| match c {
    //     '(' => acc + 1,
    //     ')' => acc - 1,
    //     _ => acc,
    // });

    println!("Day 1 Part1 solution: {}", num_list);
}
