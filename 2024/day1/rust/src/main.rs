use std::fs::read_to_string;

fn part_one() -> Option<u32> {
    let Ok(contents) = read_to_string("../input.txt") else {
        eprintln!("Failed to read input.txt");
        return None;
    };
    let n = contents.lines().count();
    let mut list1: Vec<u32> = vec![0u32; n];
    let mut list2: Vec<u32> = vec![0u32; n];
    contents
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let mut nums = line.split_whitespace();
            let num1 = nums.next().and_then(|n| n.parse::<u32>().ok());
            let num2 = nums.next().and_then(|n| n.parse::<u32>().ok());
            match (num1, num2) {
                (Some(n1), Some(n2)) => {
                    list1[idx] = n1;
                    list2[idx] = n2;
                }
                _ => {
                    eprintln!("Failed to read line correctly");
                    return false;
                }
            }
            true
        })
        .all(|x| x);
    list1.sort();
    list2.sort();
    let ans = (0..n).fold(0, |acc, idx| acc + list1[idx].abs_diff(list2[idx]));
    Some(ans)
}

fn part_two() -> Option<u32> {
    let Ok(contents) = read_to_string("../input.txt") else {
        eprintln!("Failed to read input.txt");
        return None;
    };
    let n = contents.lines().count();
    let mut list1: Vec<u32> = vec![0u32; n];
    let mut list2: Vec<u32> = vec![0u32; n];
    contents
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let mut nums = line.split_whitespace();
            let num1 = nums.next().and_then(|n| n.parse::<u32>().ok());
            let num2 = nums.next().and_then(|n| n.parse::<u32>().ok());
            match (num1, num2) {
                (Some(n1), Some(n2)) => {
                    list1[idx] = n1;
                    list2[idx] = n2;
                }
                _ => {
                    eprintln!("Failed to read line correctly");
                    return false;
                }
            }
            true
        })
        .all(|x| x);
    list1.sort();
    list2.sort();
    let ans: u32 = list1
        .iter()
        .map(|&num1| {
            let freq = list2.iter().filter(|&&num2| num1 == num2).count();
            num1 * freq as u32
        })
        .sum();
    Some(ans)
}

fn main() {
    println!("Part 1: {}", part_one().unwrap());
    println!("Part 2: {}", part_two().unwrap());
}
