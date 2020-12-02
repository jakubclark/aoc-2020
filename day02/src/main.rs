fn is_valid(line: &str) -> bool {
    let mut parts = line.split_whitespace();

    let mut range_parts = parts.next().unwrap().split("-");
    let min: usize = range_parts.next().unwrap().parse().unwrap();
    let max: usize = range_parts.next().unwrap().parse().unwrap();
    let char_to_check = parts.next().unwrap().chars().next().unwrap();
    let pass = parts.next().unwrap();

    let occurences = pass
        .chars()
        .into_iter()
        .filter_map(|c| if c == char_to_check { Some(()) } else { None })
        .count();
    occurences >= min && occurences <= max
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .filter(|line| is_valid(line))
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    println!("Part 1 solution: {}", part1);
}

#[test]
fn part1_works() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    let valid_count = part1(input);
    assert_eq!(2, valid_count);
}
