pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    (digits.iter().fold(0, |acc, &x| acc * 10 + x) + 1)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

pub fn faster_plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for digit in digits.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return digits;
        }

        *digit = 0;
    }

    digits.insert(0, 1);
    digits
}
fn main() {
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
    assert_eq!(
        plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1]
    );
}
