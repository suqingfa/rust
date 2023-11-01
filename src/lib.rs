//! rust test

/// Calculate the sum of two numbers
///
/// # Examples
/// ```
/// let sum = rust::add(1,2);
/// assert_eq!(3, sum);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(3, add(1, 2));
    }
}