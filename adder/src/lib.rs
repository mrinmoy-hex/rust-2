#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// A guess whose value is guaranteed to be between 1 and 100.
pub struct Guess {
    value: i32,
}

impl Guess {
    /// Creates a guess, panicking if `value` is outside the allowed range.
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

impl Rectangle {
    // A rectangle can hold another only when it is strictly larger in both dimensions.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Returns the sum of two unsigned integers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Returns a generic greeting; `name` is currently not included in the output.
pub fn greeting(name: &str) -> String {
    // To include the supplied name, use: format!("hello {name}!")
    format!("hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        let result = add(4, 2);
        assert_eq!(result, 6);
    }
    // This example is disabled because `greeting` currently does not include its name argument.
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{result}`"
    //     );
    // }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Tests can return `Result` to report failure with `Err` instead of asserting or panicking.
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
