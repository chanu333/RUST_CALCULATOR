#[cfg(test)]
mod tests {
    use super::operations::{add, sub, mul, div};

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 4.0), 6.0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(3.0, 2.0), 1.0);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(4.0, 2.0).unwrap(), 2.0);
    }

    #[test]
    fn test_div_by_zero() {
        assert!(div(4.0, 0.0).is_err());
    }
}
