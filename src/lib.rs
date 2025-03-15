use std::{collections::HashMap, fmt::Display};

use usize_point::Point;

pub mod direction;
pub mod grid;
pub mod grid_display;
pub mod math;
pub mod range_util;
pub mod string_grid;
pub mod usize_point;

pub fn find_chars_positions(input: &str, char_test: fn(char) -> bool) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(move |(x, c)| match char_test(c) {
                    true => Some((x, y)),
                    false => None,
                })
        })
        .collect::<Vec<(usize, usize)>>()
}

pub fn manhattan_dist(lhs: &(usize, usize), rhs: &(usize, usize)) -> usize {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1)
}

pub fn manhattan_dist_u64(lhs: &(u64, u64), rhs: &(u64, u64)) -> u64 {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1)
}

pub fn parse_input_blocks(input: &str) -> Vec<String> {
    let mut line_iter = input.lines().peekable();
    let mut result: Vec<String> = vec![];

    while line_iter.peek().is_some() {
        let block = line_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<String>();

        result.push(block);
        _ = line_iter.next()
    }

    result
}

pub fn print_hash_map_values<T: Display>(map: HashMap<Point, T>, grid_size: Point) -> String {
    let (_, longest_string) = map
        .iter()
        .max_by(|(_, a), (_, b)| a.to_string().len().cmp(&b.to_string().len()))
        .unwrap();
    let len = longest_string.to_string().len();

    let mut result: String = String::new();
    for y in 0..grid_size.1 {
        for x in 0..grid_size.0 {
            let value = map
                .get(&(x, y))
                .map_or(String::from("0"), |value| value.to_string());
            result.push_str(format!("{:0>len$}.", value, len = len).as_str());
        }
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_char_positions() {
        let result = find_chars_positions(
            "....#.
..#...
#.....",
            |c| c == '#',
        );
        assert_eq!(result, vec![(4, 0), (2, 1), (0, 2)]);
    }

    #[test]
    fn test_manhattan_dist() {
        let result = manhattan_dist(&(1, 3), &(4, 5));
        assert_eq!(result, 5)
    }

    use crate::parse_input_blocks;

    #[test]
    fn test_input_block_parsing() {
        let result = parse_input_blocks(
            "#####uaie#
uiaeuiaetröü
uiaetnuidaet

uiaentuiranes
uiatreudiaen
uiadteruiane

uiatreudiare
u",
        );

        assert_eq!(result.len(), 3)
    }
}
