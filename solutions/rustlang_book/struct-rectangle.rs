#[derive(Debug)]
struct Rectangle {
    width: u64,
    length: u64,
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

    let rectl = Rectangle {
        width: dbg!(40 * 50),
        length: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    // println!("rectl is {rect1:#?}");
    dbg!(&rectl);
}

fn area(rectangle: &Rectangle) -> u64 {
    rectangle.width * rectangle.length
}
