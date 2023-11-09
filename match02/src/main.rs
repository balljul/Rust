fn main() {
    let diceroll = 1;
    match diceroll {
        7 => println!("Got a seven"),
        2 => println!("Got a two"),
        // other => reroll(other),
        _ => reroll(0),
    }

    fn reroll(last_number: u8) {
        println!("Rolled the number {}", last_number)
    }
}
