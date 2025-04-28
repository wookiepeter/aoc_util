use crate::direction::Direction;
use crate::usize_point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub size: (usize, usize),
    pub array: Vec<Vec<T>>,
}

/*
Feature TODO:
- getter that handles out of bounds and maybe even different numeric types?!
    - way simpler would be just to have a secondary getter that uses those types
- get_neighbors that handles out of bounds
- Proper Errorhandling and Result Types to handle all the out of bounds stuff

- Direction struct
- integration of directions into neighbors?!
    - not sure if this is needed, a good direction class could just be used to access stuff
*/

impl<Char> Grid<Char> {
    pub fn new_char_grid(input: &str) -> Grid<char> {
        Grid::new(input, |c| c)
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    /// creates new Grid
    ///
    /// # Arguments
    ///
    /// * `input` - input string, appropriate line breaks are assumed and not checket!
    /// * `handle_char` - lambda that converts each char into the linked T value
    pub fn new(input: &str, handle_char: fn(char) -> T) -> Grid<T> {
        let array: Vec<Vec<T>> = input
            .lines()
            .map(|line| line.chars().map(handle_char).collect())
            .collect();

        let size = (array[0].len(), array.len());
        Grid { size, array }
    }

    /// Returns the element T at the position, if the position is  valid
    pub fn get(&self, position: (usize, usize)) -> Option<&T> {
        if self.is_inbound(position) {
            Some(&self.array[position.1][position.0])
        } else {
            None
        }
    }

    pub fn set(&mut self, position: (usize, usize), e: T) {
        if self.is_inbound(position) {
            self.array[position.1][position.0] = e;
        } else {
            panic!("Invalid position")
        }
    }

    pub fn is_inbound(&self, position: (usize, usize)) -> bool {
        position.0 < self.size.0 && position.1 < self.size.1
    }

    /// Retrieves positions of neighbors.
    ///
    /// # Warning
    ///
    /// This filters out any positions that are out of bounds, so you might end up with an empty Vec.
    pub fn get_neighbors(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        std::iter::empty()
            .chain(position.0.checked_sub(1).zip(Some(position.1)))
            .chain(Some(position.0).zip(position.1.checked_sub(1)))
            .chain(position.0.checked_add(1).zip(Some(position.1)))
            .chain(Some(position.0).zip(position.1.checked_add(1)))
            .filter(|(x, y)| *x < self.size.0 && *y < self.size.1)
            .collect()
    }

    /// Retrieves values of neighbors.
    ///
    /// # Warning
    ///
    /// This filters out any positions that are out of bounds, so you might end up with an empty Vec.
    pub fn get_neighbor_values(&self, position: (usize, usize)) -> Vec<T> {
        self.get_neighbors(position)
            .iter()
            .map(|position| self.get(*position).cloned().unwrap())
            .collect()
    }

    pub fn get_direct_neighbor(
        &self,
        position: (usize, usize),
        direction: Direction,
    ) -> Option<((usize, usize), &T)> {
        match usize_point::point_add(position, direction.into(), self.size) {
            Some(position) => self.get(position).map(|value| (position, value)),
            None => None,
        }
    }
}

impl<T> Grid<T>
where
    T: Eq + PartialEq,
{
    pub fn find_positions(&self, elem: &T) -> Vec<(usize, usize)> {
        self.array
            .iter()
            .enumerate()
            .flat_map(|(row, row_array)| {
                row_array
                    .iter()
                    .enumerate()
                    .filter_map(move |(column, value)| match value.eq(elem) {
                        true => Some((column, row)),
                        false => None,
                    })
            })
            .collect()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (usize, usize);

    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<T> Iterator for GridIter<'_, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.grid.size.0 {
            self.x = 0;
            self.y += 1;
            if self.y >= self.grid.size.1 {
                return None;
            }
        }
        let res = Some((self.x, self.y));
        self.x += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::Grid;

    #[test]
    fn test_get() {
        let char_grid = Grid::<char>::new_char_grid(
            r"ABC
DEF
GHI",
        );

        assert_eq!(char_grid.get((0, 0)), Some(&'A'));
        assert_eq!(char_grid.get((2, 0)), Some(&'C'));
        assert_eq!(char_grid.get((0, 2)), Some(&'G'));
        assert_eq!(char_grid.get((3, 0)), None);
        assert_eq!(char_grid.get((0, 3)), None);
    }

    #[test]
    fn test_set() {
        let mut char_grid = Grid::<char>::new_char_grid(
            r"ABC
DEF
GHI",
        );

        assert_eq!(char_grid.get((0, 0)), Some(&'A'));
        char_grid.set((0, 0), 'C');
        assert_eq!(char_grid.get((0, 0)), Some(&'C'));
        assert_eq!(char_grid.get((2, 1)), Some(&'F'));
        char_grid.set((2, 1), 'X');
        assert_eq!(char_grid.get((2, 1)), Some(&'X'));
    }

    #[test]
    fn test_neighbors() {
        let char_grid = Grid::<char>::new_char_grid(
            r"ABC
DEF
GHI",
        );

        assert_eq!(
            char_grid.get_neighbors((1, 1)),
            vec![(0, 1), (1, 0), (2, 1), (1, 2)]
        );

        assert_eq!(
            char_grid.get_neighbors((0, 1)),
            vec![(0, 0), (1, 1), (0, 2)]
        );

        assert_eq!(
            char_grid.get_neighbors((1, 0)),
            vec![(0, 0), (2, 0), (1, 1)]
        );

        assert_eq!(char_grid.get_neighbors((2, 2)), vec![(1, 2), (2, 1)]);
    }

    #[test]
    fn tget_neighbor_values() {
        let char_grid = Grid::<char>::new_char_grid(
            r"ABC
DEF
GHI",
        );

        assert_eq!(
            char_grid.get_neighbor_values((1, 1)),
            vec!['D', 'B', 'F', 'H']
        );

        assert_eq!(char_grid.get_neighbor_values((0, 1)), vec!['A', 'E', 'G']);

        assert_eq!(char_grid.get_neighbor_values((1, 0)), vec!['A', 'C', 'E']);

        assert_eq!(char_grid.get_neighbor_values((2, 2)), vec!['H', 'F']);
        assert_eq!(char_grid.get_neighbor_values((5, 6)), vec![]);
    }

    #[test]
    fn test_direct_neighbors() {
        let char_grid = Grid::<char>::new_char_grid(
            r"ABC
DEF
GHI",
        );

        assert_eq!(
            char_grid.get_direct_neighbor((1, 1), Direction::Up),
            Some(((1, 0), &'B'))
        );
        assert_eq!(
            char_grid.get_direct_neighbor((2, 0), Direction::Right),
            None
        );
    }

    #[test]
    fn test_grid_iter() {
        let char_grid = Grid::<char>::new_char_grid(
            r"ABC
DEF
GHI",
        );

        let positions: Vec<(usize, usize)> = char_grid.into_iter().collect();
        let test_positions = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        assert_eq!(positions, test_positions)
    }
}
