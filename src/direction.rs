use std::convert::From;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

/*
Feature List
- [x] rotation (clockwise and counter-clockwise)
- [x] opposite
- [x] Conversion to i32
    - use simple match statement
*/

impl Direction {
    pub const CLOCKWISE: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];
    pub const COUNTER_CLOCKWISE: [Self; 4] = [Self::Up, Self::Left, Self::Down, Self::Right];

    fn rotate(&self, directions: [Self; 4], times: usize) -> Self {
        let (index, _) = directions
            .iter()
            .enumerate()
            .find(|(_, dir)| *dir == self)
            .unwrap();

        directions[(index + times) % 4]
    }

    /// rotates the direction clockwise
    pub fn cw(&self) -> Self {
        self.rotate(Self::CLOCKWISE, 1)
    }

    /// rotates the direction counter-clockwise
    pub fn ccw(&self) -> Self {
        self.rotate(Self::COUNTER_CLOCKWISE, 1)
    }

    pub fn opposite(&self) -> Self {
        self.rotate(Self::CLOCKWISE, 2)
    }

    pub fn is_horizontal(&self) -> bool {
        *self == Direction::Right || *self == Direction::Left
    }

    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    #[test]
    fn test_convert() {
        let v: (i32, i32) = Direction::Up.into();

        assert_eq!((0, -1), v);
    }

    #[test]
    fn test_rotation() {
        assert_eq!(Direction::Left, Direction::Up.ccw());
        assert_eq!(Direction::Right, Direction::Up.cw());
        assert_eq!(Direction::Up, Direction::Right.cw().cw().cw());
        assert_eq!(Direction::Right, Direction::Left.opposite());
    }
}
