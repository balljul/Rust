enum Coin {
    OCENT,
    TCENT,
    FCENT,
    EURO,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::OCENT => {
            println!("You paid a singular cent");
            1
        },
        Coin::TCENT => 2,
        Coin::FCENT => 5,
        Coin::EURO => 100,
    }
}

fn main() {
    let payment = Coin::OCENT;
    println!("You payd {} cents", value_in_cents(payment));
}
