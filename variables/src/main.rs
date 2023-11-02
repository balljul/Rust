use rand::Rng;

fn main() {
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
}
