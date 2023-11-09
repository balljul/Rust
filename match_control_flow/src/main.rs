#[derive(Debug)]
#[allow(unused)]
enum UsState {
    Alabama,
    Washington,
    Texas,
}

#[derive(Debug)]
#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            1
        },
        Coin::Nickel => 2,
        Coin::Dime => 5,
        Coin::Quarter(state) => {
            println!("You got Quarter from {:#?}", state);
            100
        },
    }
}

fn main() {
    let payment = Coin::Quarter(UsState::Alabama);
    println!("You payd {} cents", value_in_cents(&payment));
    dbg!(payment);
}
