use restaurant::front_of_house

fn main() {
    let config_max = Some(15u8);

    if let Some(max) = config_max {
        println!("The max is {}", max);
    }
    else {
        println!("else");
    }
}
