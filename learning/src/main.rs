#[derive(Debug)]
struct Color(i32,i32,i32);

#[derive(Debug)]
struct Point(u64,u64,u64);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let black = Color(255,255,255);
    let origin = Point(0,0,0);
    let unit = AlwaysEqual;

    dbg!(black, origin, unit);
}
