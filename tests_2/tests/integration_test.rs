use tests_2::add_two;

mod common;

#[test]
fn public_api_adds_two_to_an_input() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4, "add_two should add two to its input");
}

#[test]
fn public_api_adds_two_to_zero() {
    common::setup();

    assert_eq!(add_two(0), 2);
}
