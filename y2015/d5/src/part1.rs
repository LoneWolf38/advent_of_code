#[allow(unused_imports)]
use std::collections::HashSet;
use std::fs;
#[allow(dead_code, unused, non_snake_case)]
fn is_vowel(c: &char) -> bool {
    matches!(*c, 'a' | 'e' | 'i' | 'o' | 'u')
}
pub fn solution(input_path: String) {
    let input = fs::read_to_string(input_path).unwrap();

    let result = input
        .lines()
        .filter(|x| {
            let mut vowels_found = vec![];

            x.chars().for_each(|ch| {
                if is_vowel(&ch) {
                    vowels_found.push(ch);
                }
            });
            if vowels_found.len() >= 3 {
                return true;
            } else {
                return false;
            }
        })
        .filter(|x| {
            let mut dupl = 0;
            x.char_indices().for_each(|(i, ch)| {
                if i < x.len() - 1 {
                    if ch == x.chars().nth(i + 1).unwrap() {
                        dupl += 1;
                    }
                }
            });
            if dupl >= 1 {
                return true;
            } else {
                return false;
            }
        })
        .filter(|x| {
            let unallowed_values = vec!["ab", "cd", "pq", "xy"];

            for value in unallowed_values.iter() {
                if x.contains(value) {
                    return false;
                }
            }

            true
        });

    println!("{:?}", result.count());
}
