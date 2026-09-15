use std::collections::HashMap;

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut result = students.len() as i32;
    let mut map: HashMap<i32, i32> = HashMap::new();

    for student in students {
        *map.entry(student).or_insert(0) += 1;
    }

    println!("{map:?}");

    for sandwich in sandwiches {
        match map.get_mut(&sandwich) {
            Some(x) if *x > 0 => {
                result -= 1;
                *x -= 1;
            }
            _ => return result,
        }
    }

    result
}

fn main() {
    todo!();
}

#[cfg(test)]
mod test {
    use crate::count_students;

    #[test]
    fn woa() {
        assert_eq!(count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
        assert_eq!(count_students(vec![1, 1], vec![0, 1]), 2)
    }
}
