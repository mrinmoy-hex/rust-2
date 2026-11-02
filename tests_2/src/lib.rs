// Test Organization
/*
 * - Two main cat: Unit tests, Integration tests
 *
 * - Unit tests are small and more focused, testing one module in isolation at a time and can test
 * private interfaces
 *
 * - Integration tests check that many parts of the library work together correctly, and they use the library’s public API to test the code in the same way external code will use it
 *
 *
 *
 */

pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
