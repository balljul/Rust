fn main() {
    let number: u32 = 6;

    if number == 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false")
    }
    
    let mut counter: u16 = 0;
    let result: u16 = { 
        loop {
            counter = counter + 1;
            println!("{counter}");
            if counter == 20 {
                break counter * 3;
            }
        }
    };

    println!("Result: {result}")

}
