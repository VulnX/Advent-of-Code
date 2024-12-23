use std::fs::read_to_string;

fn is_monotonic(levels: &Vec<u16>) -> bool {
    let is_increasing = levels.windows(2).all(|x| x[0] < x[1]);
    let is_decreasing = levels.windows(2).all(|x| x[0] > x[1]);
    if is_increasing || is_decreasing {
        true
    } else {
        false
    }
}

fn satisfy_condition(levels: &Vec<u16>) -> bool {
    if !is_monotonic(&levels) {
        false
    } else {
        levels.windows(2).all(|x| {
            let diff = x[0].abs_diff(x[1]);
            1 <= diff && diff <= 3
        })
    }
}

fn part_one() -> Option<usize> {
    let Ok(contents) = read_to_string("../input.txt") else {
        eprintln!("Failed to read input file");
        return None;
    };
    let ans = contents
        .lines()
        .filter(|line| {
            let levels: Vec<u16> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<u16>().ok())
                .collect();
            satisfy_condition(&levels)
        })
        .count();
    Some(ans)
}

fn part_two() -> Option<usize> {
    let Ok(contents) = read_to_string("../input.txt") else {
        eprintln!("Failed to read input file");
        return None;
    };
    let ans = contents
        .lines()
        .filter(|line| {
            let levels: Vec<u16> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<u16>().ok())
                .collect();
            if satisfy_condition(&levels) {
                return true;
            } else {
                (0..levels.len()).any(|i| {
                    let substituted_levels: Vec<u16> = levels
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &val)| if i == j { None } else { Some(val) })
                        .collect();
                    satisfy_condition(&substituted_levels)
                })
            }
        })
        .count();
    Some(ans)
}

fn main() {
    println!("Part 1: {}", part_one().unwrap());
    println!("Part 2: {}", part_two().unwrap());
}
