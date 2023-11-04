fn _loop_testing() {
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

fn main() {

    'capital_loop: loop { 
        let kapitel = 3;
        let mut subcap = 0;
        
        '_subcapitel_loop: loop {
            println!("Kapitel: {kapitel}.{subcap}");
            subcap += 1;
            if subcap == 17 {
                break 'capital_loop;
            }
        }
        
    }


}

// Notes to myself:
// Loop Labels are usefull if you want to break or continue a loop at a higher scope
// If conditions without the () takes a bit of getting used to
