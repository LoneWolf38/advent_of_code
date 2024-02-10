use std::collections::HashSet;
use std::fs;
#[allow(dead_code, unused, non_snake_case)]
pub fn solution(input_path: String) {
    let lines = fs::read_to_string(input_path).unwrap();
    let mut houses_hashset = HashSet::from([(0, 0)]);

    let mut current_position: (i32, i32) = (0, 0);

    lines.chars().for_each(|c| match c {
        '>' => {
            houses_hashset.insert((current_position.0 + 1, current_position.1));
            current_position = (current_position.0 + 1, current_position.1);
        }
        '<' => {
            houses_hashset.insert((current_position.0 - 1, current_position.1));
            current_position = (current_position.0 - 1, current_position.1);
        }
        '^' => {
            houses_hashset.insert((current_position.0, current_position.1 + 1));
            current_position = (current_position.0, current_position.1 + 1);
        }
        'v' => {
            houses_hashset.insert((current_position.0, current_position.1 - 1));
            current_position = (current_position.0, current_position.1 - 1);
        }
        _ => (),
    });

    println!("Day 3 Part1 solution: {}", houses_hashset.len());
}
