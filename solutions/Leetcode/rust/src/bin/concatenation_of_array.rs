pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    nums.repeat(2)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::get_concatenation;

    #[test]
    fn test_concatenation() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
        assert_eq!(
            get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
