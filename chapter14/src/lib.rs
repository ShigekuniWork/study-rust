//! # My Crate
//!
//! `add_one` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// use add_one;
/// let five = 5;
/// assert_eq!(6, add_one::add_one(5));
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

