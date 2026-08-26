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

pub fn big_theta_n_plus_k_smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    // Step 1: creating a frequency table for a range of values
    // 0..100
    // to store the values freqeuncy in the range
    //
    let mut count = vec![0; 100];
    for &num in nums.iter() {
        count[num as usize] += 1;
    }

    // Step 2: Implementing the prefix sum of the frequencies
    // This shows the counter of the values in the backend
    let mut running_sum = 0;
    for i in 0..100 {
        let temp = count[i];
        count[i] = running_sum;
        running_sum += temp;
    }

    nums.into_iter().map(|num| count[num as usize]).collect()
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::{big_theta_n_plus_k_smaller_numbers_than_current, smaller_numbers_than_current};

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
    #[test]
    fn test_smaller_num_4() {
        assert_eq!(
            big_theta_n_plus_k_smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
