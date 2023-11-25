fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    &largest
}

fn main() {

    let my_list = [23, 3732, 34, 532, 99, 999, 9999, 10450];
    
    println!("{}", largest(&my_list));

    let my_list = vec![2,3,4,5,6,70,8];

    println!("{}", largest(&my_list));
    
}
