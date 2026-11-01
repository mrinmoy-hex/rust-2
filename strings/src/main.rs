// Return a borrowed slice containing everything before the first ASCII space.
fn first_word(s: &str) -> &str {
    // Searching bytes is safe here because the delimiter is ASCII, so its index
    // is always a valid UTF-8 character boundary.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // A `String` can be borrowed as `&str`; the returned word borrows the same text.
    let word = first_word(&my_string[0..6]);
    println!("First word: {word}");

    let mut hello = String::from("नमस्ते");
    // `push_str` appends text without replacing the existing contents.
    hello.push_str(" in hindi.");

    println!("Updated string: {hello}");
    // Rust strings are UTF-8: bytes are not necessarily the same as characters.
    println!("UTF-8 bytes in the string:");
    for byte in hello.as_bytes() {
        println!("byte = {byte}");
    }

    // `+` takes ownership of the left-hand `String`; `s2` is borrowed for the append.
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("Concatenated string: {s3}");

    // `format!` combines multiple strings without taking ownership of its arguments.
    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");

    let s = format!("{str1}-{str2}-{str3}");
    println!("Formatted string: {s}");
}
