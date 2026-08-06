#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    pub fn mixup<X, Y>(&self, other_point: Point<X, Y>) -> Point<&T, Y> {
        Point {
            x: &self.x,
            y: other_point.y,
        }
    }
}

pub fn impl_testing() {
    let p1 = Point { x: 5, y: 64.6 };
    let p2 = Point {
        x: "Hello world",
        y: "umpalumapa",
    };

    let p3 = p1.mixup(p2);

    println!("p3.x = {} and p3.y = {}", p3.x, p3.y);
}

pub fn largest_number<T: PartialOrd + Copy>(vector_list: Vec<T>) -> T {
    let mut num = vector_list[0];

    for i in vector_list {
        if i > num {
            num = i
        }
    }

    num
}
