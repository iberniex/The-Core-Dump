use std::collections::VecDeque;

pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let mut queue: VecDeque<(usize, i32)> = tickets.into_iter().enumerate().collect();
    let k = k as usize;

    let mut counter: i32 = 0;

    while let Some((i, remaining)) = queue.pop_front() {
        counter += 1;
        let remaining = remaining - 1;

        if i == k && remaining == 0 {
            return counter;
        }

        if remaining > 0 {
            queue.push_back((i, remaining));
        }
    }

    counter
}
fn main() {}

#[test]
fn test_uwu() {
    assert_eq!(time_required_to_buy(vec![2, 3, 2], 2), 6);
    assert_eq!(time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
}
