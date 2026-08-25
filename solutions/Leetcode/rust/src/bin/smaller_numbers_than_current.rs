pub fn smaller_numbers_than_current(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; nums.len()];
    for i in 0..nums.len() {
        let temp = nums[i];
        nums.remove(i);
        for num in 0..nums.len() {
            if nums[num] < temp {
                result[i] += 1;
            }
        }

        nums.insert(i, temp);
    }

    result
}

pub fn faster_smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for &num in nums.iter() {
        result.push(nums.iter().filter(|&&x| x < num).count() as i32);
    }

    result
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::smaller_numbers_than_current;

    #[test]
    fn test_smaller_num_1() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }
    #[test]
    fn test_smaller_num_2() {
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
    }
    #[test]
    fn test_smaller_num_3() {
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
