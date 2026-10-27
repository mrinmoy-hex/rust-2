// use std::fs::File;

fn main() {
    // panic!("crash and burn");
    let _v = vec![1, 2, 3];
    //  v[99]; //buffer overread
    // recoverable errors
    // let r = divide(4, 0).unwrap_or(-1);
    let r = match divide(4, 0) {
        Ok(num) => num,
        Err(err) => {
            println!("[Error] {err}");
            -1
        }
    };
    println!("R = {r}");
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Don't divide by zero."));
    }

    Ok(x / y)
}
