#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Tree,
}

#[derive(Debug)]
struct Grid(Vec<Vec<Tile>>);

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let tiles: Vec<Vec<_>> = input
            .lines()
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Tree,
                        '.' => Tile::Empty,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Self(tiles)
    }
}

impl Grid {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    fn count_trees_for_traversal(
        &self,
        starting_x: usize,
        starting_y: usize,
        x_step_size: usize,
        y_step_size: usize,
    ) -> u32 {
        let mut current_x = starting_x;
        let mut current_y = starting_y;
        let height = self.height();
        let width = self.width();
        let mut count = 0;
        while current_y < height {
            let curr = self.0[current_y][current_x];
            current_x = (current_x + x_step_size) % width;
            current_y += y_step_size;
            if curr == Tile::Tree {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = Grid::from(input);

    let part1 = grid.count_trees_for_traversal(0, 0, 3, 1);
    println!("Part 1 solution: {}", part1);
}

#[test]
fn part1_works() {
    let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    let grid = Grid::from(input);
    let encountered_trees = grid.count_trees_for_traversal(0, 0, 3, 2);
    assert_eq!(7, encountered_trees);
}
