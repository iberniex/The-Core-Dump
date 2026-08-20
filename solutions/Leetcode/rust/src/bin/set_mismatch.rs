use std::collections::HashSet;

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();
    let mut duplicate = 0;

    for n in &nums {
        if !set.insert(n) {
            duplicate = *n;
        }
    }
    let nums = nums.len() as i32;

    let ready = (1..=nums)
        // .into_iter()
        // .filter(|x| !set.contains(&x))
        .find(|x| !set.contains(&x))
        .unwrap_or(0);

    vec![duplicate, ready]
}

pub fn faster_find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    let mut duplicate = -1;

    for i in 0..nums.len() {
        let idx = (nums[i].abs() - 1) as usize;

        if nums[idx] < 0 {
            duplicate = nums[idx].abs()
        } else {
            nums[idx] = -nums[idx]
        }
    }

    let missing = (0..nums.len())
        .find(|&x| nums[x] > 0)
        .map(|x| (x + 1) as i32)
        .unwrap_or(0);

    vec![duplicate, missing]
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::{faster_find_error_nums, find_error_nums};

    #[test]
    fn test_set_mismatch() {
        assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
        assert_eq!(find_error_nums(vec![2, 2]), vec![2, 1]);
        assert_eq!(find_error_nums(vec![3, 2, 3, 4, 6, 5]), vec![3, 1]);
        assert_eq!(faster_find_error_nums(vec![3, 2, 3, 4, 6, 5]), vec![3, 1]);
    }
}
