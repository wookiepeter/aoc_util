pub type Point = (usize, usize);

/// Contains a bunch of helper functions for working with usize points
/// mostly useful for working with 2d arrays!

pub fn point_add(
    point: Point,
    direction: (i32, i32),
    bounds: (usize, usize),
) -> Option<(usize, usize)> {
    let Some(x) = point.0.checked_add_signed(direction.0 as isize) else {
        return None; 
    }; 
    let Some(y) = point.1.checked_add_signed(direction.1 as isize) else {
        return None; 
    }; 
    if x >= bounds.0 || y >= bounds.1 {
        return None; 
    }
    Some((x, y))
}