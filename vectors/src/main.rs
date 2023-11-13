#[allow(unused)]
fn main() {
    let mut v = vec![];

    for num in (1..=20) {
        v.push(num);
    }
    dbg!(&v);
    
    let third: &i32 = &v[2];
    println!("Third item is: {}", third);

    let third: Option<&i32> = v.get(10);
    match third {
        Some(value) => println!("The 10th value is {value}"),
        None => println!("No 10th value"),
    }

}
