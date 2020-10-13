use std::collections::HashMap;

pub fn manipulating_hash_maps() {
    let mut scores = HashMap::new();
    scores.insert("dc", 156);
    scores.insert("rcb", 190);

    let entry = scores.entry("rcb");
    println!("{:?}", entry);

    entry.or_insert(200);
    println!("{:?}", scores);
}

pub fn hash_map_using_collect() {
    let teams = vec!["blue", "red", "green"];
    let scores = vec![134, 154];
    
    let team_scores: HashMap<&str, i32> = teams.into_iter().zip(scores.into_iter()).collect();

    for item in team_scores {
        println!("{:?}", item);
    }
}

pub fn update_hashmap() {
    let text = String::from("hello world hello world arif hello");
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);
}
