use std::fs;
#[allow(dead_code, unused, non_snake_case)]
pub fn solution(input_path: String) {
    let lines = fs::read_to_string(input_path).unwrap();

    let mut total = 0;

    let presents: Vec<(u32, u32, u32)> = lines
        .lines()
        .map(|line| {
            let mut split = line.split('x');
            let l = split.next().unwrap().parse::<u32>().unwrap();
            let w = split.next().unwrap().parse::<u32>().unwrap();
            let h = split.next().unwrap().parse::<u32>().unwrap();
            (l, w, h)
        })
        .collect();
    total = presents
        .iter()
        .map(|(l, w, h)| {
            let mut sides = vec![l * w, w * h, h * l];
            sides.sort();
            2 * sides[0] + 2 * sides[1] + 2 * sides[2] + sides[0]
        })
        .sum::<u32>();

    println!("Day 2 Part1 solution: {:?}", total);
}
