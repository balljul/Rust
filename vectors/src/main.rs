#[allow(unused)]
fn main() {
    let mut v = vec![];

    for num in (1..=12) {
        v.push(num);
    }

    for item in &mut v {
        *item += 50;
    }
    
    dbg!(&v);
    
    let third: &i32 = &v[2];
    println!("Third item is: {}", third);

    let third: Option<&i32> = v.get(10);
    match third {
        Some(value) => println!("The 10th value is {value}"),
        None => println!("No 10th value"),
    }

    let value_hundred: Option<&i32> = v.get(100);
    match value_hundred {
        Some(value) => println!("The 100th value is: {}", value),
        None => println!("There is no 100th value"),
    }

}
