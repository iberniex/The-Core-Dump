#[derive(Debug)]
struct Rectangle {
    width: u64,
    length: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.length
    }

    fn compare(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width < other.width
    }
}

// Associated functions
impl Rectangle {
    fn square(val: u64) -> Rectangle {
        Rectangle {
            width: val,
            length: val,
        }
    }
}

fn main() {
    // refactoring
    // let width = 30;
    // let height = 50;

    // refactoring two
    // let rect = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle {
        width: dbg!(40 * 50),
        length: 50,
    };

    let rect3 = Rectangle::square(24);

    println!("rect1 compared to rect2 {}", &rect1.compare(&rect2));

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("rectl is {rect1:#?}");

    println!("rect3 is {:#?}", rect3);
}
