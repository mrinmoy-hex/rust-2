// a closure is an anonymous function that can capture vars from the scope in which its' defined

// fn expensive_calculation() -> u32 {
//     println!("Calculating...");
//     10
// }
/*

let variable = |parameters| {
    // closure body
};

- closures don't require type annotations

*/
fn execute<F>(action: F) -> String
where
    F: FnOnce() -> String,
{
    action()
}

fn main() {
    // a closure assigned to a variable
    let calculate = || {
        println!("Calculating...");
        10
    };

    // execute the closure
    let result = calculate();

    println!("{result}");

    let name = String::from("Mrinmoy");

    let print_name = || {
        // accessing a var from the outer scope
        println!("Hello, {name}");
    };

    print_name();

    // capturing the env: borrowing and ownership

    // immutable borrow
    let lst = vec![1, 2, 3];

    // closure only reads the vector
    let print_lst = || {
        println!("{:?}", lst);
    };

    print_lst();

    // we can still access list
    println!("Outside closure: {:?}", lst);

    // mutable borrow
    let mut lst_2 = vec![1, 2, 3];

    // closure needs to modify list
    let mut modify_list = || {
        // modify the original vector
        lst_2.push(2);
    };

    // call the closure
    modify_list();

    println!("{:?}", lst_2);

    // taking ownerships with move
    let print_name_second = move || {
        println!("Hello, {name}");
    };

    print_name_second();

    // ERROR: name was moved into the closure
    // println!("{name}");

    /*
    Three closures traits: Fn, FnMut and FnOnce

     */

    let message = || String::from("Hello Rust!");

    let result = execute(message);

    println!("{result}");
}
