pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut ans: Vec<i32> = vec![0; temperatures.len()];

    for (i, &temp) in temperatures.iter().enumerate() {
        while let Some(&x) = stack.last() {
            if temperatures[x] >= temp {
                break;
            }
            // get the value between the current index and the value in the stack
            // which is the amount of days between
            ans[x] = (i - x) as i32;
            stack.pop();
        }

        stack.push(i)
    }

    ans
}
fn main() {
    todo!();
}

#[cfg(test)]
mod test {
    use crate::daily_temperatures;

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        )
    }
    #[test]
    fn test_daily_temperatures_1() {
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0])
    }

    #[test]
    fn test_daily_temperatures_2() {
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0])
    }
}
