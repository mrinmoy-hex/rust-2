//! Small examples of Rust's unit-test attributes and assertions.
//!
//! Tests in one process normally run in parallel. Keep each test independent:
//! avoid relying on another test's changes to shared state, the current
//! directory, or environment variables. Use a mutex or serial test setup only
//! when a test genuinely needs shared state.

/// Prints the input value and returns a fixed value for assertion examples.
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

/// Returns the sum of two unsigned integers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_the_sum() {
        let result = add(2, 2);
        assert_eq!(result, 4, "adding two and two should produce four");
    }

    #[test]
    fn add_handles_zero_and_different_operands() {
        assert_eq!(add(0, 7), 7);
        assert_eq!(add(3, 5), 8);
    }

    #[test]
    fn assert_eq_can_check_a_value_returned_by_a_helper() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    // This intentionally failing example is ignored during normal `cargo test`
    // runs. Run ignored tests explicitly with `cargo test -- --ignored`.
    fn ignored_example_shows_a_failing_assertion() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}
