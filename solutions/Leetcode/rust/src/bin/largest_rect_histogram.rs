pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut area: i32 = 0;
    let mut stack: Vec<(usize, i32)> = vec![];

    for (i, &h) in heights.iter().enumerate() {
        let mut start = i;

        while let Some(&x) = stack.last() {
            if x.1 < h {
                break;
            }

            area = area.max(x.1 * (i - x.0) as i32);

            start = x.0;
        }

        stack.push((start, h))
    }

    for (index, height) in stack {
        area = area.max(height * (heights.len() - index) as i32)
    }
    area
}
fn main() {
    todo!();
}

#[cfg(test)]
mod test {
    use crate::largest_rectangle_area;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10)
    }
    #[test]
    fn test_largest_rectangle_area_2() {
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4)
    }
    #[test]
    fn test_largest_rectangle_area_3() {
        assert_eq!(largest_rectangle_area(vec![0, 9]), 9)
    }
}
