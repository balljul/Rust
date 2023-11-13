#[allow(unused)]
fn main() {
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(243),
        Cell::Float(27.2833),
        Cell::Text(String::from("Some random string")),
    ];

    dbg!(&row);

}

#[allow(unused)]
fn vectors() {

    let mut v = vec![];

    for num in (1..=12) {
        v.push(num);
    }

    for item in &mut v {
        *item *= *item;
    }
    
    dbg!(&v);
    
    let third: &i32 = &v[2];
    println!("Third item is: {}", third);

    let third: Option<&i32> = v.get(9);
    match third {
        Some(value) => println!("The 10th value is {value}"),
        None => println!("No 10th value"),
    }

    let value_hundred: Option<&i32> = v.get(99);
    match value_hundred {
        Some(value) => println!("The 100th value is: {}", value),
        None => println!("There is no 100th value"),
    }
    
}
