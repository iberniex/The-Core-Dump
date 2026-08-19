pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut counter = 0;
    let mut buf_counter = 0;

    for (i, &value) in nums.iter().enumerate() {
        if i == 0 {
            if value == 1 {
                counter += 1;
                buf_counter += 1;
                continue;
            } else {
                continue;
            }
        }

        if value == nums[i - 1] && value == 1 {
            counter += 1;
        } else {
            if value == 1 {
                if counter > buf_counter {
                    buf_counter = counter;
                }

                counter = 1;
            }
        }
    }

    if counter > buf_counter {
        return counter;
    }

    buf_counter
}

pub fn faster_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    nums.split(|&num| num != 1)
        .map(|v| v.len())
        .max()
        .unwrap_or(0) as i32
}

fn main() {
    todo!();
}

#[cfg(test)]
mod test {
    use crate::find_max_consecutive_ones;

    #[test]
    fn test_cfind_max_consecutive_ones() {
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
        assert_eq!(find_max_consecutive_ones(vec![0]), 0);
        assert_eq!(find_max_consecutive_ones(vec![1]), 1);
        assert_eq!(find_max_consecutive_ones(vec![0, 0]), 0);
        assert_eq!(find_max_consecutive_ones(vec![0, 0, 0]), 0);
    }
}
