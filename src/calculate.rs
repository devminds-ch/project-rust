//! # Calculate Module
//!
//! This module provides basic mathematical calculation functions.

/// Adds two floating-point numbers together.
///
/// # Arguments
///
/// * `a` - The first number to add
/// * `b` - The second number to add
///
/// # Returns
///
/// Returns the sum of `a` and `b` as a `f64`
///
/// # Examples
///
/// ```
/// use rust_training_project::calculate::sum;
///
/// let result = sum(2.5, 3.7);
/// assert_eq!(result, 6.2);
/// ```
pub fn sum(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    //! Tests for the calculate module functions.

    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(1.0, 1.0), 2.0);
        assert_eq!(sum(2.0, 1.0), 3.0);
        assert_eq!(sum(1.0, 2.0), 3.0);
    }

    #[test]
    fn test_sum_negative() {
        assert_eq!(sum(1.0, -1.0), 0.0);
        assert_eq!(sum(-1.0, 1.0), 0.0);
        assert_eq!(sum(-1.0, -1.0), -2.0);
    }
}
