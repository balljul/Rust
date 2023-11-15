fn main() {
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
