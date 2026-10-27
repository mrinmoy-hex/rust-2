fn first_word(s: &str) -> &str {
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

    let word = first_word(&my_string[0..6]);

    let mut hello = String::from("नमस्ते");
    hello.push_str(" in hindi.");

    // concatenating strings
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    //
    // using format macro for complicated strings
    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");

    let s = format!("{str1}-{str2}-{str3}");

    for c in hello.as_bytes() {
        println!("c = {c}");
    }
}
