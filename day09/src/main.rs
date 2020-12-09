fn is_sum_of_two(number: u64, preamble: &[u64]) -> bool {
    preamble.into_iter().any(|&n1| {
        preamble
            .into_iter()
            .any(|&n2| n1 != n2 && n1 + n2 == number)
    })
}

fn solve_part1(input: &str, preamble_len: usize) -> u64 {
    let input = input.lines().collect::<Vec<_>>();

    let mut preamble = input[..preamble_len]
        .into_iter()
        .map(|&line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    let numbers = input[preamble_len..]
        .into_iter()
        .map(|&line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    numbers
        .into_iter()
        .find(|&n| {
            let result = !is_sum_of_two(n, preamble.as_slice());
            preamble.remove(0);
            preamble.push(n);
            result
        })
        .unwrap()
}

fn solve_part2(input: &str, invalid_number: u64) -> u64 {
    let numbers = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    for i in 0..numbers.len() {
        let mut subset = vec![numbers[i]];
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            subset.push(numbers[j]);
            if sum == invalid_number {
                let min = subset.iter().min().unwrap();
                let max = subset.iter().max().unwrap();
                return min + max;
            }
        }
    }

    unreachable!()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input, 25);
    println!("Part 1 solution: {}", part1);
    let part2 = solve_part2(input, part1);
    println!("Part 1 solution: {}", part2);
}

#[test]
fn part1_works() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let part1 = solve_part1(input, 5);
    assert_eq!(127, part1);
}

#[test]
fn part2_works() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let part2 = solve_part2(input, 127);
    assert_eq!(62, part2);
}
