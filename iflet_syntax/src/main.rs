fn main() {
    let config_max = Some(15u8);
    match config_max {
        Some(max) => println!("The max is {}", max),
        _ => (),
    }
}
