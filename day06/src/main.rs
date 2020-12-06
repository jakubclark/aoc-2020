use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1 solution: {}", part1);

    let part2 = solve_part2(input);
    println!("Part 2 solution: {}", part2);
}

/// A list of group-answers.
/// A group answer, contains the set of answered question each person answered
fn split_by_group(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut result = vec![];
    for line in input.lines() {
        if line.is_empty() {
            result.push(Vec::new());
        } else if let Some(last) = result.last_mut() {
            last.push(line.chars().collect::<HashSet<char>>());
        } else {
            result.push(vec![line.chars().collect()]);
        }
    }

    result
}

fn solve_part1(input: &str) -> usize {
    split_by_group(input)
        .into_iter()
        .map(|group| group.into_iter().flatten().collect::<HashSet<char>>().len())
        .sum()
}

fn count_same_answers(group: &[HashSet<char>]) -> usize {
    let mut answers = HashSet::new();
    group.iter().for_each(|person1| {
        person1
            .into_iter()
            .filter(|&answer| group.iter().all(|person2| person2.contains(answer)))
            .for_each(|answer| {
                let _ = answers.insert(answer);
            });
    });
    answers.len()
}

fn solve_part2(input: &str) -> usize {
    split_by_group(input)
        .into_iter()
        .map(|group| count_same_answers(group.as_slice()))
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

#[test]
fn part2_works() {
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
    let part2 = solve_part2(input);
    assert_eq!(6, part2);
}
