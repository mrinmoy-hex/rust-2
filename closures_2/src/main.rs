fn main() {
    let list = vec![1, 2, 3];
    println!("before defining closure: {list:?}");

    let only_borrows = || println!("from closure: {list:?}");

    println!("before calling closure: {list:?}");
    only_borrows(); // vars can bind closures definition
    println!("after calling closure: {list:?}");

    println!("New Test");

    let mut lst = vec![1, 2, 3];
    println!("before defining closure: {list:?}");

    let mut borrows_mutably = || lst.push(7);

    println!("this is just a text ");
    borrows_mutably();
    println!("after calling closure: {list:?}");
}
