use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1 solution: {}", part1);
}

fn split_by_group(input: &str) -> Vec<String> {
    let mut result = vec![String::new()];
    for line in input.lines() {
        if line.is_empty() {
            result.push(String::new());
        } else if let Some(last) = result.last_mut() {
            last.push_str(line);
        } else {
            result.push(String::from(line));
        }
    }
    result
}

/// Get the unique answered questions,for the given group
fn get_answered_questions(input: &str) -> HashSet<char> {
    input.lines().flat_map(|line| line.chars()).collect()
}

/// Count the number of answered questions, for the given group
fn count_answered_questions(input: &str) -> usize {
    get_answered_questions(input).len()
}

fn solve_part1(input: &str) -> usize {
    split_by_group(input)
        .into_iter()
        .map(|group_answers| count_answered_questions(group_answers.as_str()))
        .sum()
}

#[test]
fn part1_works() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let part1 = solve_part1(input);
    assert_eq!(11, part1);
}
