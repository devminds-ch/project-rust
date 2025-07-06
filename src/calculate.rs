pub fn sum(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
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
