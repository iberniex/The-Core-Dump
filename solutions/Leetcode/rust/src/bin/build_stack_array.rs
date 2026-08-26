pub fn build_array_operations(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut s = vec![];
    let mut stream_of_integers: Vec<i32> = (1..=n).collect();

    let mut i: usize = 0;
    while i < target.len() {
        if let Some(&x) = target.get(i) {
            if x == stream_of_integers[i] {
                s.push(String::from("Push"));
                i += 1;
            } else {
                s.push(String::from("Push"));
                s.push(String::from("Pop"));
                stream_of_integers.remove(i);
            }
        }
    }

    s
}

pub fn faster_build_array_operations(target: Vec<i32>, _n: i32) -> Vec<String> {
    let mut s = vec![];
    let mut current = 1;

    for &num in target.iter() {
        while current < num {
            s.push(String::from("Push"));
            s.push(String::from("Pop"));
            current += 1;
        }
        s.push(String::from("Push"));
        current += 1;
    }

    s
}
fn main() {}

#[cfg(test)]
mod test {

    use crate::{build_array_operations, faster_build_array_operations};

    #[test]
    fn test_build_array_operations() {
        assert_eq!(
            build_array_operations(vec![1, 3], 3),
            vec![
                String::from("Push"),
                String::from("Push"),
                String::from("Pop"),
                String::from("Push")
            ]
        )
    }

    #[test]
    fn test_build_array_operations_1() {
        assert_eq!(
            build_array_operations(vec![1, 2, 3], 3),
            vec![
                String::from("Push"),
                String::from("Push"),
                String::from("Push")
            ]
        )
    }
    #[test]
    fn test_build_array_operations_2() {
        assert_eq!(
            build_array_operations(vec![1, 2], 4),
            vec![String::from("Push"), String::from("Push"),]
        )
    }
    #[test]
    fn test_build_array_operations_3() {
        assert_eq!(
            build_array_operations(vec![2, 3, 4], 4),
            vec![
                String::from("Push"),
                String::from("Pop"),
                String::from("Push"),
                String::from("Push"),
                String::from("Push")
            ]
        )
    }
    #[test]
    fn test_build_array_operations_5() {
        assert_eq!(
            faster_build_array_operations(vec![2, 3, 4], 4),
            vec![
                String::from("Push"),
                String::from("Pop"),
                String::from("Push"),
                String::from("Push"),
                String::from("Push")
            ]
        )
    }
}
