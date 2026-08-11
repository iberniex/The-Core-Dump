pub fn removing_elements(arr: &[u8]) -> Vec<u8> {
    // arr.iter()
    //     .enumerate()
    //     .filter(|(index, _)| index % 2 == 0)
    //     .map(|(_, value)| *value)
    //     .collect()
    //
    arr.iter().step_by(2).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            removing_elements(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            &[1, 3, 5, 7, 9]
        );
    }
}
