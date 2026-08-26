pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        let idx = (nums[i].abs() - 1) as usize;

        if nums[idx] > 0 {
            nums[idx] = -nums[idx]
        }
    }

    let mut result = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        if nums[i] > 0 {
            result.push((i + 1) as i32);
        }
    }

    result
}

pub fn cyclic_sort_numbers_disappeared(mut nums: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < nums.len() {
        let correct = (nums[i] - 1) as usize;
        if nums[i] != nums[correct] {
            let temp = nums[i];
            nums[i] = nums[correct];
            nums[correct] = temp;
        } else {
            i += 1;
        }
    }

    (0..nums.len())
        .filter(|&x| nums[x] != (x + 1) as i32)
        .map(|x| (x + 1) as i32)
        .collect()
}

pub fn fastest_solution_sort_numbers_disappeared(nums: Vec<i32>) -> Vec<i32> {
    let mut miss: Vec<i32> = (0..(nums.len() as i32)).collect();
    for num in nums {
        miss[(num - 1) as usize] = 0;
    }

    miss.into_iter().filter(|&x| x != 0).collect()
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::{cyclic_sort_numbers_disappeared, find_disappeared_numbers};

    #[test]
    fn test_numbers_disappeared_1() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        )
    }

    #[test]
    fn test_numbers_disappeared_2() {
        assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2])
    }
    #[test]
    fn test_numbers_disappeared_3() {
        assert_eq!(
            find_disappeared_numbers(vec![10, 2, 5, 10, 9, 1, 1, 4, 3, 7]),
            vec![6, 8]
        )
    }
    #[test]
    fn test_numbers_disappeared_4_cyclic_sort() {
        assert_eq!(
            cyclic_sort_numbers_disappeared(vec![10, 2, 5, 10, 9, 1, 1, 4, 3, 7]),
            vec![6, 8]
        )
    }
    #[test]
    fn test_numbers_disappeared_5_cyclic_sort() {
        assert_eq!(cyclic_sort_numbers_disappeared(vec![1, 1]), vec![2])
    }
}
