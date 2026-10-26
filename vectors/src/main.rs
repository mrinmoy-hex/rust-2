fn main() {
    // Vectors allows to store more than one value in a single data structure, stored in heap
    // can store values of the same type

    // creating empty vector
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    // updating a vector
    let mut v1 = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    // reading elements of vectors
    let mut v2 = vec![1, 2, 3, 4, 5];

    // via indexing => goes into panic if index goes out of bound
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    // using get method and Option => returns of Option<&t> and if index goes out of bound returns
    // None
    let second: Option<&i32> = v2.get(1);
    match second {
        Some(second) => println!("The second element is {second}"),
        _ => (),
    }

    // iterating over the values in vector
    for i in &v2 {
        println!("{i}");
    }

    // iterating over mutable
    println!("Iterating over mutable and changing it's value");
    for i in &mut v2 {
        // to change the value we have to dereference i first
        *i += 50;
        println!("{i}");
    }

    // Using enum to store multiple types
    enum SpredsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpredsheetCell::Int(3),
        SpredsheetCell::Text(String::from("blue")),
        SpredsheetCell::Float(9.11),
    ];

    {
        let v3 = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed here
}
