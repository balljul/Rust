fn main() {
    let number: u32 = 6;

    if number == 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false")
    }
    
    let mut counter: u16 = 0;
    loop {
        println!("{counter}");
        counter = counter + 1;
        if counter == 20 {
            break;
        }
    }

    println!("Result: {counter}")

}
