fn valid_count(line: &str) -> bool {
    let mut parts = line.split_whitespace();

    let mut range_parts = parts.next().unwrap().split('-');
    let min: usize = range_parts.next().unwrap().parse().unwrap();
    let max: usize = range_parts.next().unwrap().parse().unwrap();
    let char_to_check = parts.next().unwrap().chars().next().unwrap();
    let pass = parts.next().unwrap();

    let occurences = pass
        .chars()
        .into_iter()
        .filter(|&c| c == char_to_check)
        .count();
    occurences >= min && occurences <= max
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .filter(|&line| valid_count(line))
        .count()
}

fn valid_position(line: &str) -> bool {
    let mut parts = line.split_whitespace();

    let mut range_parts = parts.next().unwrap().split('-');
    let first: usize = range_parts.next().unwrap().parse().unwrap();
    let second: usize = range_parts.next().unwrap().parse().unwrap();
    let char_to_check = parts.next().unwrap().chars().next().unwrap().to_string();
    let pass = parts.next().unwrap();

    let first = &pass[first - 1..first];
    let second = &pass[second - 1..second];

    (first == char_to_check && second != char_to_check)
        || (first != char_to_check && second == char_to_check)
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .filter(|&line| valid_position(line))
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    println!("Part 1 solution: {}", part1);
    let part2 = part2(input);
    println!("Part 2 solution: {}", part2);
}

#[test]
fn part1_works() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    let valid_count = part1(input);
    assert_eq!(2, valid_count);
}

#[test]
fn part2_works() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    let valid_count = part2(input);
    assert_eq!(1, valid_count);
}
