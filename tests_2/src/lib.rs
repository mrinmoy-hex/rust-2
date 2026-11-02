//! Examples of unit and integration test organization.
//!
//! Unit tests usually live beside the code they exercise, so they can test
//! private helpers. Integration tests live under `tests/` and use the crate's
//! public API, just as a downstream program would.

/// Adds two to `a` using the private [`internal_adder`] helper.
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

/// Adds two values; kept private to demonstrate testing an internal helper.
fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

/// Adds two unsigned integers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_the_sum() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn private_helper_returns_the_sum() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn public_wrapper_adds_two() {
        assert_eq!(add_two(5), 7);
    }
}
