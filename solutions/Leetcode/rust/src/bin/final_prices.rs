pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut ans: Vec<i32> = prices.clone();
    for i in 0..prices.len() {
        while !stack.is_empty() && prices[i] <= prices[*(stack.last().unwrap())] {
            let index = stack.pop().unwrap();
            ans[index] = prices[index] - prices[i];
        }

        stack.push(i);
    }

    ans
}
fn main() {
    println!("[8, 7, 4, 2, 8, 1, 7, 7, 10, 1]");
    assert_eq!(final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    assert_eq!(
        final_prices(vec![8, 7, 4, 2, 8, 1, 7, 7, 10, 1]),
        vec![1, 3, 2, 1, 7, 0, 0, 6, 9, 1]
    );
}

#[cfg(test)]
mod test {
    use crate::final_prices;

    #[test]
    fn test_final_prices() {
        assert_eq!(final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
    }
    #[test]
    fn test_final_prices2() {
        assert_eq!(final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_final_prices3() {
        assert_eq!(final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
    #[test]
    fn test_final_prices4() {
        assert_eq!(
            final_prices(vec![8, 7, 4, 2, 8, 1, 7, 7, 10, 1]),
            vec![1, 3, 2, 1, 7, 0, 0, 6, 9, 1]
        );
    }
}
