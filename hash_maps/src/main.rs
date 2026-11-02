use std::{collections::HashMap, hash::Hash};

fn main() {
    // Create a map and insert key/value pairs. Inserting a key that already
    // exists replaces its old value.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values in a hash map
    let team_name = String::from("Blue");
    // `get` returns an Option<&V>; `copied` turns the copied integer into an
    // Option<i32>, and `unwrap_or` supplies a value when the key is absent.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    // Hash-map iteration order is not guaranteed, so do not rely on the order
    // in which these entries are printed.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Inserting an existing key replaces its value and returns the old one.
    let replaced_score = scores.insert(String::from("Blue"), 25);
    println!("replaced Blue score: {replaced_score:?}");

    // A map may borrow its keys and values. The referenced strings must stay
    // alive for at least as long as this map is used.
    let field_name = String::from("Favourite linux distro");
    let field_value = String::from("Arch linux");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{map:#?}");

    // `entry` keeps an existing score unchanged and inserts the default only
    // when the key is missing.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);

    println!("{scores:?}");

    // A common counting pattern mutates the value returned by `or_insert`.
    let words = ["blue", "yellow", "blue"];
    let mut word_counts = HashMap::new();
    for word in words {
        *word_counts.entry(word).or_insert(0) += 1;
    }
    println!("word counts: {word_counts:?}");
}
