#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale: u32 = 2;
    let rect01 = Rectangle {
        width: dbg!(30 * scale),
        height: 40,
    };
    dbg!(rect01);

    println!(
        "The are of the width is: {}",
        area(&rect01)
        );

    println!(
        "The instance looks like this: \n{:#?}",
        rect01
        );
}

fn area(rectangle: &Rectangle) -> u32 {
   rectangle.width * rectangle.height 
}
