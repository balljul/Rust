fn main() {
    println!("Hello, world!");
    
    let result = {
        let x = test_function(4, 8);
        x + 1
    };

    println!("The result of the expression plus 1 is: {result}");
}

fn test_function(number: u32, multiplier: u32) -> u32  {
    number * multiplier
}

// Last section was 3.3.Parameters

// Notes for me:
// Statements are actions that perform an aciton and don´t return a value
// Expressions are actions that end in an returned value
