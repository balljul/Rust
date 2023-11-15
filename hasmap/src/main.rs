fn main() {
    use std::collections::HashMap;
    
    { 
    let mut hmap: HashMap<String, i32> = HashMap::new();

    hmap.insert(String::from("test"), 255);
    hmap.insert(String::from("test"), 124);

    hmap.entry(String::from("KAC")).or_insert(700);
    hmap.entry(String::from("KAC")).or_insert(900);
    }

    let mut hmap = HashMap::new();
    let words = String::from("This is just sequence of random words and letters which could be: f a a sj dk k d v u d");

    for word in words.split_whitespace() {
        let count = hmap.entry(word).or_insert(0); 
        *count += 1;
    }

    dbg!(&hmap);
}

#[allow(unused)]
fn hasmaps_test() {

    use std::collections::HashMap;
    let mut scores = HashMap::new();

    let score_kac: i32 = 99;
    let score_vsv: i32 = 0;
    let score_test: i32 = 4;

    scores.insert(String::from("KAC"), score_kac);
    scores.insert(String::from("VSV"), score_vsv);
    scores.insert(String::from("test"), score_test);
        
    dbg!(&scores);

    let team_name = String::from("KAC");
    let score = scores.get(&team_name).copied();

    match score {
        Option::Some(value) => println!("KAC has {} points this season", value),
        Option::None => println!("KAC doesnt have a valid entry"),
    }
}
