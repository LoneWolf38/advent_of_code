use std::collections::HashSet;
use std::fs;
#[allow(dead_code, unused, non_snake_case)]
pub fn solution(input_path: String) {
    let lines = fs::read_to_string(input_path).unwrap();
    let mut santa_houses_hashset = HashSet::from([(0, 0)]);
    let mut robo_santa_house_set = HashSet::from([(0, 0)]);

    let mut santa_current_position: (i32, i32) = (0, 0);
    let mut robo_santa_current_position: (i32, i32) = (0, 0);

    lines.chars().enumerate().for_each(|(i, c)| match c {
        '>' => {
            if i % 2 != 0 {
                santa_houses_hashset
                    .insert((santa_current_position.0 + 1, santa_current_position.1));
                santa_current_position = (santa_current_position.0 + 1, santa_current_position.1);
            } else {
                robo_santa_house_set.insert((
                    robo_santa_current_position.0 + 1,
                    robo_santa_current_position.1,
                ));
                robo_santa_current_position = (
                    robo_santa_current_position.0 + 1,
                    robo_santa_current_position.1,
                );
            }
        }
        '<' => {
            if i % 2 != 0 {
                santa_houses_hashset
                    .insert((santa_current_position.0 - 1, santa_current_position.1));
                santa_current_position = (santa_current_position.0 - 1, santa_current_position.1);
            } else {
                robo_santa_house_set.insert((
                    robo_santa_current_position.0 - 1,
                    robo_santa_current_position.1,
                ));
                robo_santa_current_position = (
                    robo_santa_current_position.0 - 1,
                    robo_santa_current_position.1,
                );
            }
        }
        '^' => {
            if i % 2 != 0 {
                santa_houses_hashset
                    .insert((santa_current_position.0, santa_current_position.1 + 1));
                santa_current_position = (santa_current_position.0, santa_current_position.1 + 1);
            } else {
                robo_santa_house_set.insert((
                    robo_santa_current_position.0,
                    robo_santa_current_position.1 + 1,
                ));
                robo_santa_current_position = (
                    robo_santa_current_position.0,
                    robo_santa_current_position.1 + 1,
                );
            }
        }
        'v' => {
            if i % 2 != 0 {
                santa_houses_hashset
                    .insert((santa_current_position.0, santa_current_position.1 - 1));
                santa_current_position = (santa_current_position.0, santa_current_position.1 - 1);
            } else {
                robo_santa_house_set.insert((
                    robo_santa_current_position.0,
                    robo_santa_current_position.1 - 1,
                ));
                robo_santa_current_position = (
                    robo_santa_current_position.0,
                    robo_santa_current_position.1 - 1,
                );
            }
        }
        _ => (),
    });

    println!("{:?}", santa_houses_hashset);
    println!("{:?}", robo_santa_house_set);

    println!(
        "Day 4 Part2 solution: {}",
        santa_houses_hashset.union(&robo_santa_house_set).count()
    );
}
