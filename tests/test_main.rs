#[cfg(test)]
mod tests {
    use calculator::arithemetics_funs::*;

    #[test]
    fn test_add() {
        assert_eq!(add(&1.0, &2.0), 3.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(&1.0, &2.0), -1.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(&1.0, &2.0), 2.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(&1.0, &2.0), 0.5);
    }
}
