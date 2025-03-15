use super::grid::*;

/// Struct to manage the displaying of a char grid and potentially modifying said grid
/// with additional chars
pub struct GridDisplay {
    size: (usize, usize),
    display_vecs: Vec<Vec<char>>,
}

/*
TODO: this should possibly be integrated with Grid<T> itself, but since it requires the
use of chars and Grid<T> can't be specified properly (see specilization RFC), a better
solution remains to be found.
-> one possibility would be to
 */
impl GridDisplay {
    pub fn new(
        default_char: char,
        size: (usize, usize),
        other_chars: Vec<(char, Vec<(usize, usize)>)>,
    ) -> Self {
        let mut display_vecs = vec![vec![default_char; size.0]; size.1];

        other_chars.iter().for_each(|(c, positions)| {
            positions
                .iter()
                .filter(|position| _in_bounds(&size, position))
                .for_each(|(column, row)| {
                    display_vecs[*row][*column] = *c;
                })
        });

        Self { size, display_vecs }
    }

    pub fn from_grid(grid: &Grid<char>) -> Self {
        Self {
            size: grid.size,
            display_vecs: grid.array.clone(),
        }
    }

    /// Replaces all chars at the provided positions with c
    pub fn apply_char_layer<I>(&mut self, c: char, positions: I)
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        positions
            .into_iter()
            .filter(|position| _in_bounds(&self.size, position))
            .for_each(|(column, row)| {
                self.display_vecs[row][column] = c;
            })
    }

    /// Replaces the char at the provided position with c
    pub fn apply_char(&mut self, c: char, position: (usize, usize)) {
        if _in_bounds(&self.size, &position) {
            self.display_vecs[position.1][position.0] = c;
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for GridDisplay {
    fn to_string(&self) -> String {
        let display_string: Vec<String> = self
            .display_vecs
            .iter()
            .map(|vec| vec.iter().collect::<String>())
            .collect();

        display_string.join("\n")
    }
}

fn _in_bounds(bounds: &(usize, usize), value: &(usize, usize)) -> bool {
    value.0 < bounds.0 && value.1 < bounds.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let display = GridDisplay::new(
            '.',
            (4, 4),
            vec![('#', vec![(0, 1), (1, 3)]), ('0', vec![(0, 0), (3, 3)])],
        );

        assert_eq!(
            display.to_string(),
            "0...
#...
....
.#.0"
        )
    }

    #[test]
    fn test_apply_layer() {
        let mut display = GridDisplay::new('.', (4, 4), vec![]);

        display.apply_char_layer('#', vec![(0, 1), (2, 3)]);
        display.apply_char_layer('!', vec![(2, 3), (1, 0), (3, 3)]);
        assert_eq!(
            display.to_string(),
            ".!..
#...
....
..!!"
        )
    }
}
