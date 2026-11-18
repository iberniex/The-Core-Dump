use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::from(stones);

    while max_heap.len() > 1 {
        let y = max_heap.pop().unwrap();
        let x = max_heap.pop().unwrap();

        if x != y {
            max_heap.push(y - x);
        }
    }

    if max_heap.is_empty() {
        return 0;
    }

    max_heap.pop().unwrap()
}
fn main() {
    todo!();
}

#[test]
fn uwu_testing_uwu() {
    assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(last_stone_weight(vec![1]), 1);
}
