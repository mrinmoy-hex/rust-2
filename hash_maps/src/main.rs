use std::{collections::HashMap, hash::Hash};

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // managing ownerships in hash maps
    let field_name = String::from("Favourite linux distro");
    let field_value = String::from("Arch linux");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{map:#?}");

    //
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}
