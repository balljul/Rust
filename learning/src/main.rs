#[derive(Debug)]
#[allow(unused)]
// tuple structs
struct Color(i32,i32,i32);

#[derive(Debug)]
#[allow(unused)]
// field structs
struct User {
    field1: bool,
    field2: u128,
}

#[derive(Debug)]
#[allow(unused)]
// unit-like structs
struct AlwaysEqual;

// Notes end here

#[derive(Debug)]
#[allow(unused)]
struct Rectangle {
    width: u32,
    heigth: u32
}

#[allow(unused)]
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
    fn compare(&self, other: Rectangle) -> bool {
        self.area() > other.area()
    }
    fn sqaure(size: u32) -> Self {
        Self {
            width: size,
            heigth: size,
        }
    }
}

#[allow(unused)]
fn main() {
    const SCALE: u32 = 2;
    let rect1 = Rectangle {
        width: 30 * SCALE,
        heigth: 50,
    };

    let rect2 = Rectangle {
        width: 18 * SCALE,
        heigth: 50,
    };

    let rect3 = Rectangle::sqaure(115);
    dbg!(&rect3);
}

