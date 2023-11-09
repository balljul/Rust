fn main() {
    let diceroll = 1;
    match diceroll {
        7 => println!("Got a seven"),
        2 => println!("Got a two"),
        // other => reroll(other),
        _ => (),
    }

    fn _reroll(last_number: u8) {
        println!("Rolled the number {}", last_number)
    }
}

// Notes to myself:
// In the match expression the catch all arm catches all options we didnt cover
// other: with other I have to use the value
// _: with _ I dont have to use the value
