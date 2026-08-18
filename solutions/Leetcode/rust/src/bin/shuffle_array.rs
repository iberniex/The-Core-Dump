pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let half_slice = (n - 1) as usize;
    println!("{half_slice}");
    let lhs = &nums[0..=half_slice];
    let rhs = &nums[half_slice + 1..];

    for i in 0..=half_slice {
        println!("{} - {}", lhs[i], rhs[i]);
        result.push(lhs[i]);
        result.push(rhs[i]);
    }

    result
}

pub fn fast_shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;

    nums[..n]
        .iter()
        .zip(&nums[n..])
        .flat_map(|(&a, &b)| [a, b])
        .collect()
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::{fast_shuffle, shuffle};

    #[test]
    fn test_suffling() {
        assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
        assert_eq!(
            shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(fast_shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
        assert_eq!(
            fast_shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        );
        assert_eq!(
            fast_shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(fast_shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }
}
