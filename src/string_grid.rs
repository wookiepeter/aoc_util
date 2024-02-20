pub struct GridInfo {
    pub default_char: char,
    pub dimensions: (usize, usize),
    pub other_chars: Vec<(char, Vec<(usize, usize)>)>,
}

impl GridInfo {
    /// Creates a String containing a 2d grid of characters with the provided positions.
    ///
    /// Creates char vecs fitting the dimensions and fills them with default char.
    /// Iterates through lists in otherchars and replaces them in the array.
    ///
    /// Doesn't handle duplicate positions, will just overwrite them.
    ///
    /// # panics on positions that are out of bounds
    pub fn create_grid(&self) -> String {
        let mut display_vecs = vec![vec![self.default_char; self.dimensions.0]; self.dimensions.1];

        self.other_chars.iter().for_each(|(c, positions)| {
            positions.iter().for_each(|(column, row)| {
                display_vecs[*row][*column] = *c;
            })
        });

        let display_string: Vec<String> = display_vecs
            .iter()
            .map(|vec| vec.iter().collect::<String>())
            .collect();

        display_string.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let info = GridInfo {
            default_char: '.',
            dimensions: (4, 4),
            other_chars: vec![('#', vec![(0, 1), (1, 3)]), ('0', vec![(0, 0), (3, 3)])],
        };

        let grid_string = info.create_grid();
        assert_eq!(
            grid_string,
            "0...
#...
....
.#.0"
        )
    }
}
