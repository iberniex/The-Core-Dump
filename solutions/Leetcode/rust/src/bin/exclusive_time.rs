pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut stack: Vec<i32> = vec![];

    let mut ans: Vec<i32> = vec![0; n as usize];
    let mut pre: i32 = 0;

    for log in logs {
        let current_call: Vec<&str> = log.split(":").collect();
        let (func, call, timestamp) = (current_call[0], current_call[1], current_call[2]);
        let (i, cur) = (
            func.parse::<i32>().unwrap(),
            timestamp.parse::<i32>().unwrap(),
        );
        if call == "start" {
            if stack.len() > 0 {
                println!("{ans:?} - {stack:?}");
                ans[(stack[(stack.len() - 1) as usize]) as usize] += cur - pre
            }
            stack.push(i);
            pre = cur;
        } else {
            ans[(stack[(stack.len() - 1) as usize]) as usize] += cur - pre + 1;
            stack.pop();
            pre = cur + 1;
        }
    }
    ans
}

pub fn faster_exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut ans: Vec<usize> = vec![0; n as usize];
    let mut stack: Vec<usize> = Vec::new();
    let mut prev_t = 0;

    for log in logs {
        let parts: Vec<&str> = log.split(":").collect();
        let id = parts[0].parse::<usize>().unwrap();
        let status = parts[1].to_string();
        let time = parts[2].parse::<usize>().unwrap();

        if status == "start" {
            if let Some(&current) = stack.last() {
                ans[current] += time - prev_t - 1;
            }

            stack.push(id);
            prev_t = time;
        } else {
            ans[id] += time - prev_t + 1;
            stack.pop();
            prev_t = time;
        }
    }
    ans.into_iter().map(|x| x as i32).collect()
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::{exclusive_time, faster_exclusive_time};

    #[test]
    fn test_exclusive_time() {
        assert_eq!(
            exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "1:start:2".to_string(),
                    "1:end:5".to_string(),
                    "0:end:6".to_string()
                ]
            ),
            vec![3, 4]
        );
    }
    #[test]
    fn test_exclusive_time_1() {
        assert_eq!(
            exclusive_time(
                1,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "0:start:6".to_string(),
                    "0:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![8]
        );
    }
    #[test]
    fn test_exclusive_time_2() {
        assert_eq!(
            exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "1:start:6".to_string(),
                    "1:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![7, 1]
        );
    }
    #[test]
    fn test_exclusive_time_3() {
        assert_eq!(
            faster_exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "1:start:6".to_string(),
                    "1:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![7, 1]
        );
    }
}
