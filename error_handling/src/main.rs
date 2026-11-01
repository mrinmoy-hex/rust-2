fn main() {
    // An unrecovered panic stops the program immediately.
    // panic!("crash and burn");
    let _v = vec![1, 2, 3];
    // Indexing past the end of a vector panics; it is not a buffer overread.
    // v[99];

    // `divide` reports a recoverable failure with `Result`, so the caller chooses a fallback.
    // let r = divide(4, 0).unwrap_or(-1);
    let r = match divide(4, 0) {
        Ok(num) => num,
        Err(err) => {
            println!("[Error] {err}");
            // Use -1 as the result when division fails.
            -1
        }
    };
    println!("R = {r}");
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        // Return an error instead of attempting an invalid division.
        return Err(String::from("Don't divide by zero."));
    }

    Ok(x / y)
}
