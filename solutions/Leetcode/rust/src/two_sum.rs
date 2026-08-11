use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in 0..=nums.len() {
        println!("{:?}", nums[i]);
        let complement = target - nums[i];
        if map.contains_key(&complement) {
            return vec![map[&complement], i as i32];
        }
        map.entry(nums[i]).or_insert(i as i32);

        println!("{map:?}");
    }

    return vec![];
}

pub fn faster_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = map.get(&complement) {
            return vec![index, i as i32];
        }

        map.insert(num, i as i32);
    }

    return vec![];
}

#[cfg(test)]
mod test {
    use crate::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum::two_sum(vec![3, 2, 4], 6), vec![1, 2]);

        assert_eq!(two_sum::faster_two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
