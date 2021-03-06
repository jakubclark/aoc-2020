use std::collections::HashSet;

fn part1(nums: &HashSet<u64>) -> u64 {
    for n in nums.iter() {
        for m in nums.iter() {
            if n + m == 2020 {
                return n * m;
            }
        }
    }
    unreachable!();
}

fn part2(nums: &HashSet<u64>) -> u64 {
    for n in nums.iter() {
        for m in nums.iter() {
            for k in nums.iter() {
                if n + m + k == 2020 {
                    return n * m * k;
                }
            }
        }
    }
    unreachable!();
}

fn main() {
    let nums = include_str!("../input.txt")
        .lines()
        .into_iter()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let part1 = part1(&nums);
    println!("Part 1 solution: {}", part1);
    let part2 = part2(&nums);
    println!("Part 2 solution: {}", part2);
}
