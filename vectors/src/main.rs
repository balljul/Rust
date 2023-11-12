#[allow(unused)]
fn main() {
    let mut v = vec![];

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    dbg!(&v);
    
    let third: &i32 = &v[2];
    println!("Third item is: {}", third);

    let third: Option<&i32> = v.get(10);
    match third {
        Some(value) => println!("The 10th value is {value}"),
        None => println!("No 10th value"),
    }

}
