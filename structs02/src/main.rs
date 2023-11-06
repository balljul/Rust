#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
       self.width * self.height 
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }
}

fn main() {
    // let scale: u32 = 2;
    let rect01 = Rectangle {
        //width: dbg!(30 * scale),
        width: 30,
        height: 40,
    };

    if rect01.width() && rect01.height() {
        println!(
            "The area of the Rectangle is: {}",
            rect01.area()
            );
    }

    //println!(
    //    "The instance looks like this: \n{:#?}",
    //    rect01
    //    );

    // dbg!(rect01);
}


// Notes to myself:
//
