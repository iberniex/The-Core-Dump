#[cfg(test)]
mod test {
    use crate::generic_types;

    #[test]
    fn test_generic_types() {
        let list_of_numbers = vec![231, 23123, 123123, 12311, 112, 12312];
        assert_eq!(generic_types::largest_number(list_of_numbers), 123123);
    }
}
