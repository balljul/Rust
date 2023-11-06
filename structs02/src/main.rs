#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect01 = Rectangle {
        width: 30,
        height: 40,
    };

    println!(
        "The are of the width is: {}",
        area(&rect01)
        );

    println!(
        "The instance looks like this: \n{:#?}",
        rect01
        );
    dbg!(rect01);
}

fn area(rectangle: &Rectangle) -> u32 {
   rectangle.width * rectangle.height 
}
