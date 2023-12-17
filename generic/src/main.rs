fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }
    &largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {

    let my_list = [23, 3732, 34, 532, 99, 999, 9999, 10450];
    println!("{}", largest(&my_list));

    let my_list = vec![2,3,4,5,6,70,8];
    println!("{}", largest(&my_list));
    
    let test = Point {x:5, y:8};
    dbg!(test);
    
    let test = Point {x:5.0, y:8.000};
    dbg!(test);
    
    let test = Point {x:5, y:8.000};
    dbg!(test);
}
