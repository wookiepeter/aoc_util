use num::{
    integer::{gcd, lcm},
    Integer,
};

/// Calculates the lowest common multiple for a vector of Integers
///
/// This is uses the functionality of the num crate
pub fn lcm_vec<T: Integer>(num_vec: Vec<T>) -> Option<T> {
    num_vec.into_iter().reduce(|acc, value| lcm(acc, value))
}

/// Calculates the greatest common divisor for a vector of Integers
///
/// This is uses the functionality of the num crate
pub fn gcd_vec<T: Integer>(num_vec: Vec<T>) -> Option<T> {
    num_vec.into_iter().reduce(|acc, value| gcd(acc, value))
}
