use std::io;
use rand::Rng;

fn testing() {
    const LINK_TO_MY_LAST_PAGE: &str = "https://doc.rust-lang.org/book/ch03-02-data-types.html";
    println!("Last Page I was on: {LINK_TO_MY_LAST_PAGE}");

    println!("\n\n");

    let x = 10;
    let mut y = rand::thread_rng().gen_range(1..=20);
    
    println!("The variable x equals {x}");
    println!("The random number generated was: {y} \nSo x + y = {}", x + y);
    println!("Now lets regenerate y");
    
    y = rand::thread_rng().gen_range(30..=100);
    
    println!("y is now {y}");
    println!("Now lets create a constant z");

    const Z: u32 = 20;
    println!("Z equals: {Z}"); 
    println!("x + y - z = {}", x + y - Z);

    println!("\n\n");

    {
        let x = x * x;
        println!("In this inner scope x will be schadowed and become x²: {x}");
    }
    println!("After the scope it wont be shadowed: {x}");

    let testnum: u8 = 255;
    let float_num: f64 = 4.23;
    println!("{testnum}");
    println!("{float_num}");

    let tuple: (u32, f64, char) = (999, 1.23, 'a'); // Tuple can contain multiple different datatypes and cant be changed
    let _first_tuple_value = tuple.0;
    let (_tup1, _tup2, _tup3) = tuple;


    let _array = [1,2,3,4,5,6,7]; // Array must contain only one datatype and size can´t change
    let array_values: [i8; 5] = [1,2,3,4,5]; // This is how I can define an array
    let _array_predefined_values = [127; 5]; // This is how I can define an array which contains the value127 5 times
    let _first_array_value = array_values[0];                                              
}

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
