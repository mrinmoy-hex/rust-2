fn main() {
    use std::collections::HashMap;

    let text = "hello world this is mrinmoy deka";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
