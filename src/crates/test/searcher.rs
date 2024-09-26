#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_another_example() {
        let result = String::from("Hello");
        assert_eq!(result, "Hello");
    }
}
