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
}
