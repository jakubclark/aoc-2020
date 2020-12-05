fn compute_number(value: &str, bit_indicator: char) -> u32 {
    value
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(i, c)| {
            if c == bit_indicator {
                Some(u32::pow(2, i as u32))
            } else {
                None
            }
        })
        .sum()
}

fn compute_row(input: &str) -> u32 {
    compute_number(input, 'B')
}

fn compute_column(input: &str) -> u32 {
    compute_number(input, 'R')
}

fn compute_seat_id(entry: &str) -> u32 {
    let (row_str, col_str) = entry.split_at(7);
    let row = compute_row(row_str);
    let col = compute_column(col_str);
    row * 8 + col
}

fn max_seat_id(input: &str) -> u32 {
    input.lines().map(compute_seat_id).max().unwrap()
}

fn get_my_seat_id(input: &str) -> u32 {
    let seat_ids: Vec<_> = input.lines().map(compute_seat_id).collect();
    let max = seat_ids.iter().max().unwrap();
    let min = seat_ids.iter().min().unwrap();
    let sum: u32 = seat_ids.iter().sum();
    (max * (max + 1) / 2) - sum - ((min - 1) * min / 2)
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = max_seat_id(input);
    println!("Part 1 solution: {}", part1);
    let part2 = get_my_seat_id(input);
    println!("Part 2 solution: {}", part2);
}

#[test]
fn part1_works() {
    let input = "BFFFBBFRRR";
    let id = compute_seat_id(input);
    assert_eq!(567, id);

    let input = "FFFBBBFRRR";
    let id = compute_seat_id(input);
    assert_eq!(119, id);

    let input = "BBFFBBFRLL";
    let id = compute_seat_id(input);
    assert_eq!(820, id);
}
